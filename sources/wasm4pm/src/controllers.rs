use crate::crypto::{verify_ed25519_signature, verify_jcs_receipt_signature};

#[derive(Debug, Clone, Copy)]
pub struct Init;

#[derive(Debug, Clone, Copy)]
pub struct Active;

#[derive(Debug, Clone, Copy)]
pub struct Quarantined;

#[derive(Debug, Clone, Copy)]
pub struct Decommissioned;

pub struct GovToken {
    pub governor_pk: [u8; 32],
}

impl GovToken {
    pub fn verify(governor_pk: &[u8; 32], signature: &[u8; 64], message: &[u8]) -> Option<Self> {
        if verify_ed25519_signature(governor_pk, signature, message) {
            Some(Self { governor_pk: *governor_pk })
        } else {
            None
        }
    }

    pub fn verify_jcs(governor_pk: &[u8; 32], signature: &[u8; 64], jcs_json: &str) -> Option<Self> {
        if verify_jcs_receipt_signature(governor_pk, signature, jcs_json) {
            Some(Self { governor_pk: *governor_pk })
        } else {
            None
        }
    }
}

pub struct ProcessController<State> {
    pub state: State,
    pub queue_capacity: u32,
    pub log_level: String,
    pub governor_pk: [u8; 32],
}

impl ProcessController<Init> {
    pub fn new(governor_pk: [u8; 32]) -> Self {
        Self {
            state: Init,
            queue_capacity: 100,
            log_level: "INFO".to_string(),
            governor_pk,
        }
    }

    pub fn transition_active(self) -> ProcessController<Active> {
        ProcessController {
            state: Active,
            queue_capacity: self.queue_capacity,
            log_level: self.log_level,
            governor_pk: self.governor_pk,
        }
    }
}

impl ProcessController<Active> {
    // Elastic adjustments (no GovToken required)
    pub fn adjust_queue_capacity(&mut self, new_capacity: u32) {
        self.queue_capacity = new_capacity;
    }

    pub fn set_log_level(&mut self, level: &str) {
        self.log_level = level.to_string();
    }

    // Compliance-critical transitions (require GovToken)
    pub fn transition_quarantine(self, token: &GovToken) -> Option<ProcessController<Quarantined>> {
        if token.governor_pk == self.governor_pk {
            Some(ProcessController {
                state: Quarantined,
                queue_capacity: self.queue_capacity,
                log_level: self.log_level,
                governor_pk: self.governor_pk,
            })
        } else {
            None
        }
    }

    pub fn transition_decommission(self, token: &GovToken) -> Option<ProcessController<Decommissioned>> {
        if token.governor_pk == self.governor_pk {
            Some(ProcessController {
                state: Decommissioned,
                queue_capacity: 0,
                log_level: "OFF".to_string(),
                governor_pk: self.governor_pk,
            })
        } else {
            None
        }
    }
}

impl ProcessController<Quarantined> {
    // Compliance-critical transitions (require GovToken)
    pub fn transition_active(self, token: &GovToken) -> Option<ProcessController<Active>> {
        if token.governor_pk == self.governor_pk {
            Some(ProcessController {
                state: Active,
                queue_capacity: self.queue_capacity,
                log_level: self.log_level,
                governor_pk: self.governor_pk,
            })
        } else {
            None
        }
    }

    pub fn transition_decommission(self, token: &GovToken) -> Option<ProcessController<Decommissioned>> {
        if token.governor_pk == self.governor_pk {
            Some(ProcessController {
                state: Decommissioned,
                queue_capacity: 0,
                log_level: "OFF".to_string(),
                governor_pk: self.governor_pk,
            })
        } else {
            None
        }
    }
}
