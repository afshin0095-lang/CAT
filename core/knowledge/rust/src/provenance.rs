use std::collections::BTreeSet;

use crate::EvidenceRef;

/// Deterministic provenance chain assembled from evidence references.
///
/// Evidence is sorted by source, reference, then observation time so identical
/// knowledge snapshots produce identical provenance order independent of insertion order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvenanceChain {
    entries: Vec<EvidenceRef>,
}

impl ProvenanceChain {
    pub fn from_evidence<I>(evidence: I) -> Self
    where
        I: IntoIterator<Item = EvidenceRef>,
    {
        let mut entries = evidence.into_iter().collect::<Vec<_>>();
        entries.sort_by(|a, b| {
            a.source
                .cmp(&b.source)
                .then_with(|| a.reference.cmp(&b.reference))
                .then_with(|| a.observed_at_ms.cmp(&b.observed_at_ms))
        });
        entries.dedup_by(|a, b| {
            a.source == b.source && a.reference == b.reference && a.observed_at_ms == b.observed_at_ms
        });
        Self { entries }
    }

    pub fn entries(&self) -> &[EvidenceRef] { &self.entries }

    pub fn sources(&self) -> BTreeSet<&str> {
        self.entries.iter().map(|entry| entry.source.as_str()).collect()
    }

    pub fn observed_at_bounds(&self) -> Option<(u64, u64)> {
        let first = self.entries.first()?.observed_at_ms;
        let last = self.entries.last()?.observed_at_ms;
        Some((first, last))
    }

    pub fn is_empty(&self) -> bool { self.entries.is_empty() }
}

#[cfg(test)]
mod tests {
    use super::ProvenanceChain;
    use crate::EvidenceRef;

    #[test]
    fn provenance_is_sorted_and_deduplicated() {
        let a = EvidenceRef { source: "z".into(), reference: "2".into(), observed_at_ms: 20 };
        let b = EvidenceRef { source: "a".into(), reference: "1".into(), observed_at_ms: 10 };
        let chain = ProvenanceChain::from_evidence([a.clone(), b.clone(), a]);
        assert_eq!(chain.entries(), &[b, EvidenceRef { source: "z".into(), reference: "2".into(), observed_at_ms: 20 }]);
    }

    #[test]
    fn source_and_time_bounds_are_derived() {
        let chain = ProvenanceChain::from_evidence([
            EvidenceRef { source: "catalog".into(), reference: "x".into(), observed_at_ms: 30 },
            EvidenceRef { source: "feed".into(), reference: "y".into(), observed_at_ms: 10 },
        ]);
        assert_eq!(chain.sources().len(), 2);
        assert_eq!(chain.observed_at_bounds(), Some((30, 10)));
    }
}
