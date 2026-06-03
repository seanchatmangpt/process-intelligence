use crate::crypto::Blake3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtelError {
    InvalidMagic,
    InvalidVersion,
    OutOfBounds,
    Utf8Error,
    NullPointer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtelSpan {
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub span_name: String,
    pub start_time_unix_us: i64,
    pub end_time_unix_us: i64,
    pub instruction_count: i64,
    pub blake3_receipt: String,
    // Standard-compliant process intelligence span metadata
    pub instance_id: String,
    pub activity_name: String,
    pub activity_type: Option<String>,
    pub lifecycle: String,
    pub token_state_before: Option<String>,
    pub token_state_after: Option<String>,
    pub witness_id: String,
    pub witness_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtelTrace {
    pub trace_id: String,
    pub event_chain_root: String,
    pub spans: Vec<OtelSpan>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum JsonToken {
    CurlyOpen,
    CurlyClose,
    BracketOpen,
    BracketClose,
    Colon,
    Comma,
    String(String),
    Number(i64),
    Null,
    Bool(bool),
}

fn tokenize(s: &str) -> Result<Vec<JsonToken>, &'static str> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            '{' => {
                tokens.push(JsonToken::CurlyOpen);
                chars.next();
            }
            '}' => {
                tokens.push(JsonToken::CurlyClose);
                chars.next();
            }
            '[' => {
                tokens.push(JsonToken::BracketOpen);
                chars.next();
            }
            ']' => {
                tokens.push(JsonToken::BracketClose);
                chars.next();
            }
            ':' => {
                tokens.push(JsonToken::Colon);
                chars.next();
            }
            ',' => {
                tokens.push(JsonToken::Comma);
                chars.next();
            }
            '"' => {
                chars.next(); // consume opening quote
                let mut string = String::new();
                let mut escaped = false;
                loop {
                    match chars.next() {
                        Some('\\') if !escaped => {
                            escaped = true;
                        }
                        Some('"') if !escaped => {
                            break;
                        }
                        Some(ch) => {
                            string.push(ch);
                            escaped = false;
                        }
                        None => return Err("Unterminated string"),
                    }
                }
                tokens.push(JsonToken::String(string));
            }
            '-' | '0'..='9' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch == '-' || ch.is_ascii_digit() {
                        num_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val = num_str.parse::<i64>().map_err(|_| "Invalid number")?;
                tokens.push(JsonToken::Number(val));
            }
            't' | 'f' | 'n' => {
                let mut word = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_lowercase() {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match word.as_str() {
                    "true" => tokens.push(JsonToken::Bool(true)),
                    "false" => tokens.push(JsonToken::Bool(false)),
                    "null" => tokens.push(JsonToken::Null),
                    _ => return Err("Invalid identifier"),
                }
            }
            _ => return Err("Unexpected character"),
        }
    }
    Ok(tokens)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum JsonValue {
    Object(std::collections::BTreeMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    String(String),
    Number(i64),
    Bool(bool),
    Null,
}

struct Parser<'a> {
    tokens: &'a [JsonToken],
    index: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&JsonToken> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&JsonToken> {
        let tok = self.tokens.get(self.index);
        if tok.is_some() {
            self.index += 1;
        }
        tok
    }

    fn parse_value(&mut self) -> Result<JsonValue, &'static str> {
        match self.next() {
            Some(JsonToken::CurlyOpen) => self.parse_object(),
            Some(JsonToken::BracketOpen) => self.parse_array(),
            Some(JsonToken::String(s)) => Ok(JsonValue::String(s.clone())),
            Some(JsonToken::Number(n)) => Ok(JsonValue::Number(*n)),
            Some(JsonToken::Bool(b)) => Ok(JsonValue::Bool(*b)),
            Some(JsonToken::Null) => Ok(JsonValue::Null),
            _ => Err("Expected JSON value"),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, &'static str> {
        let mut map = std::collections::BTreeMap::new();
        if let Some(JsonToken::CurlyClose) = self.peek() {
            self.next();
            return Ok(JsonValue::Object(map));
        }
        loop {
            let key = match self.next() {
                Some(JsonToken::String(s)) => s.clone(),
                _ => return Err("Expected string key in object"),
            };
            match self.next() {
                Some(JsonToken::Colon) => {}
                _ => return Err("Expected ':' after key"),
            }
            let val = self.parse_value()?;
            map.insert(key, val);

            match self.next() {
                Some(JsonToken::Comma) => {}
                Some(JsonToken::CurlyClose) => break,
                _ => return Err("Expected ',' or '}' in object"),
            }
        }
        Ok(JsonValue::Object(map))
    }

    fn parse_array(&mut self) -> Result<JsonValue, &'static str> {
        let mut list = Vec::new();
        if let Some(JsonToken::BracketClose) = self.peek() {
            self.next();
            return Ok(JsonValue::Array(list));
        }
        loop {
            let val = self.parse_value()?;
            list.push(val);
            match self.next() {
                Some(JsonToken::Comma) => {}
                Some(JsonToken::BracketClose) => break,
                _ => return Err("Expected ',' or ']' in array"),
            }
        }
        Ok(JsonValue::Array(list))
    }
}

impl OtelTrace {
    pub fn parse_from_str(s: &str) -> Result<Self, &'static str> {
        let tokens = tokenize(s)?;
        let mut parser = Parser { tokens: &tokens, index: 0 };
        let root_val = parser.parse_value()?;
        if parser.index < tokens.len() {
            return Err("Trailing tokens after JSON root");
        }
        
        let root_map = match root_val {
            JsonValue::Object(m) => m,
            _ => return Err("Root must be a JSON object"),
        };
        
        let trace_id = match root_map.get("trace_id") {
            Some(JsonValue::String(s)) => s.clone(),
            _ => return Err("Missing or invalid trace_id"),
        };
        
        let event_chain_root = match root_map.get("event_chain_root") {
            Some(JsonValue::String(s)) => s.clone(),
            _ => return Err("Missing or invalid event_chain_root"),
        };
        
        let spans_val = match root_map.get("spans") {
            Some(JsonValue::Array(a)) => a,
            _ => return Err("Missing or invalid spans array"),
        };
        
        let mut spans = Vec::new();
        for val in spans_val {
            let span_map = match val {
                JsonValue::Object(m) => m,
                _ => return Err("Span must be a JSON object"),
            };
            
            let span_id = match span_map.get("span_id") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => return Err("Missing or invalid span_id"),
            };
            
            let parent_span_id = match span_map.get("parent_span_id") {
                Some(JsonValue::String(s)) => Some(s.clone()),
                Some(JsonValue::Null) | None => None,
                _ => return Err("Invalid parent_span_id"),
            };
            
            let span_name = match span_map.get("span_name") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => return Err("Missing or invalid span_name"),
            };
            
            let start_time_unix_us = match span_map.get("start_time_unix_us") {
                Some(JsonValue::Number(n)) => *n,
                _ => return Err("Missing or invalid start_time_unix_us"),
            };
            
            let end_time_unix_us = match span_map.get("end_time_unix_us") {
                Some(JsonValue::Number(n)) => *n,
                _ => return Err("Missing or invalid end_time_unix_us"),
            };
            
            let instruction_count = match span_map.get("instruction_count") {
                Some(JsonValue::Number(n)) => *n,
                _ => return Err("Missing or invalid instruction_count"),
            };
            
            let blake3_receipt = match span_map.get("blake3_receipt") {
                Some(JsonValue::String(s)) => s.clone(),
                _ => return Err("Missing or invalid blake3_receipt"),
            };

            let empty_map = std::collections::BTreeMap::new();
            let attrs_map = match span_map.get("attributes") {
                Some(JsonValue::Object(m)) => m,
                _ => &empty_map,
            };

            let get_attr = |key: &str| -> Option<String> {
                attrs_map.get(key)
                    .or_else(|| span_map.get(key))
                    .and_then(|v| match v {
                        JsonValue::String(s) => Some(s.clone()),
                        _ => None,
                    })
            };

            let instance_id = get_attr("process.pi.instance_id")
                .or_else(|| get_attr("instance_id"))
                .unwrap_or_else(|| "inst_legacy".to_string());

            let activity_name = get_attr("process.pi.activity.name")
                .or_else(|| get_attr("activity_name"))
                .unwrap_or_else(|| span_name.clone());

            let activity_type = get_attr("process.pi.activity.type")
                .or_else(|| get_attr("activity_type"));

            let lifecycle = get_attr("process.pi.lifecycle")
                .or_else(|| get_attr("lifecycle"))
                .unwrap_or_else(|| "complete".to_string());

            let token_state_before = get_attr("process.pi.token.state_before")
                .or_else(|| get_attr("token_state_before"));

            let token_state_after = get_attr("process.pi.token.state_after")
                .or_else(|| get_attr("token_state_after"));

            let witness_id = get_attr("process.pi.witness.id")
                .or_else(|| get_attr("witness_id"))
                .unwrap_or_else(|| "witness_legacy".to_string());

            let witness_hash = get_attr("process.pi.witness.hash")
                .or_else(|| get_attr("witness_hash"))
                .unwrap_or_else(|| blake3_receipt.clone());
            
            spans.push(OtelSpan {
                span_id,
                parent_span_id,
                span_name,
                start_time_unix_us,
                end_time_unix_us,
                instruction_count,
                blake3_receipt,
                instance_id,
                activity_name,
                activity_type,
                lifecycle,
                token_state_before,
                token_state_after,
                witness_id,
                witness_hash,
            });
        }
        
        Ok(OtelTrace {
            trace_id,
            event_chain_root,
            spans,
        })
    }
}

#[allow(clippy::too_many_arguments)]
pub fn hash_span(
    prior_hash: Option<&[u8; 32]>,
    trace_id: &str,
    span_id: &str,
    parent_span_id: Option<&str>,
    span_name: &str,
    start_time: i64,
    end_time: i64,
    instruction_count: i64,
    instance_id: &str,
    activity_name: &str,
    activity_type: Option<&str>,
    lifecycle: &str,
    token_state_before: Option<&str>,
    token_state_after: Option<&str>,
    witness_id: &str,
    witness_hash: &str,
) -> [u8; 32] {
    let mut hasher = Blake3::new();
    if let Some(prior) = prior_hash {
        hasher.update(prior);
    }
    hasher.update(trace_id.as_bytes());
    hasher.update(span_id.as_bytes());
    hasher.update(parent_span_id.unwrap_or("").as_bytes());
    hasher.update(span_name.as_bytes());
    hasher.update(&start_time.to_le_bytes());
    hasher.update(&end_time.to_le_bytes());
    hasher.update(&instruction_count.to_le_bytes());
    hasher.update(instance_id.as_bytes());
    hasher.update(activity_name.as_bytes());
    hasher.update(activity_type.unwrap_or("").as_bytes());
    hasher.update(lifecycle.as_bytes());
    hasher.update(token_state_before.unwrap_or("").as_bytes());
    hasher.update(token_state_after.unwrap_or("").as_bytes());
    hasher.update(witness_id.as_bytes());
    hasher.update(witness_hash.as_bytes());
    hasher.finalize()
}

pub fn verify_otel_trace(trace: &OtelTrace) -> Result<bool, &'static str> {
    if trace.spans.is_empty() {
        return Err("Trace contains no spans");
    }

    if trace.spans.len() > 1000000 {
        return Err("Trace chain cap exceeded (max 1,000,000)");
    }

    let mut prior_hash: Option<[u8; 32]> = None;

    for span in &trace.spans {
        if span.start_time_unix_us > span.end_time_unix_us {
            return Err("Span start time cannot be after end time");
        }

        if let Some(ref parent_id) = span.parent_span_id {
            if let Some(parent_span) = trace.spans.iter().find(|s| s.span_id == *parent_id) {
                if parent_span.start_time_unix_us > span.start_time_unix_us
                    || span.end_time_unix_us > parent_span.end_time_unix_us
                {
                    return Err("Parent-child timing constraint violated");
                }
            }
        }

        let mut current_parent = span.parent_span_id.clone();
        let mut visited = std::collections::HashSet::new();
        visited.insert(span.span_id.clone());
        while let Some(ref p_id) = current_parent {
            if visited.contains(p_id) {
                return Err("Cyclic parent-child dependency detected");
            }
            visited.insert(p_id.clone());
            current_parent = trace.spans.iter()
                .find(|s| s.span_id == *p_id)
                .and_then(|s| s.parent_span_id.clone());
        }

        let computed = hash_span(
            prior_hash.as_ref(),
            &trace.trace_id,
            &span.span_id,
            span.parent_span_id.as_deref(),
            &span.span_name,
            span.start_time_unix_us,
            span.end_time_unix_us,
            span.instruction_count,
            &span.instance_id,
            &span.activity_name,
            span.activity_type.as_deref(),
            &span.lifecycle,
            span.token_state_before.as_deref(),
            span.token_state_after.as_deref(),
            &span.witness_id,
            &span.witness_hash,
        );

        let computed_hex = hex_encode(&computed);
        if computed_hex != span.blake3_receipt {
            return Err("Span BLAKE3 receipt mismatch (tampering detected)");
        }

        prior_hash = Some(computed);
    }

    if let Some(final_hash) = prior_hash {
        let final_hex = hex_encode(&final_hash);
        if final_hex != trace.event_chain_root {
            return Err("Event chain root hash mismatch");
        }
    } else {
        return Err("Unexpected empty chain state");
    }

    Ok(true)
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}
