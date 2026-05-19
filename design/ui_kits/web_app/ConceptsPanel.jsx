// ConceptsPanel — Concept Store contents.
const SAMPLE_CONCEPTS = [
  { id: "metric",     title: "Metric Spaces",         subs: ["Definition", "Metric functions"] },
  { id: "norm",       title: "Norm Spaces",           subs: ["Definition"] },
  { id: "inner",      title: "Inner Product Spaces",  subs: ["Definition"] },
  { id: "relate",     title: "Relationship between…", subs: ["Theorem 1", "Proof 1", "Theorem 2", "Proof 2"] },
  { id: "seq",        title: "Sequence and Conve…",   subs: ["Definition (Sequence)", "Definition (Convergence)", "Definition (Cauchy Seq…", "Definition (Completene…"] },
];

function ConceptsPanel({ onBuild }) {
  const [search, setSearch] = React.useState("");
  const items = SAMPLE_CONCEPTS.filter(c =>
    c.title.toLowerCase().includes(search.toLowerCase()) ||
    c.subs.some(s => s.toLowerCase().includes(search.toLowerCase()))
  );
  return (
    <Panel
      title="Concept Store"
      search={search}
      onSearch={setSearch}
      footer={
        <Button variant="primary" leadingIcon={<Icon.Plus className="ico-12" />} onClick={onBuild}>
          Build Concepts
        </Button>
      }
    >
      {items.map(c => (
        <ConceptRow key={c.id} title={c.title} subitems={c.subs} />
      ))}
    </Panel>
  );
}

window.ConceptsPanel = ConceptsPanel;
window.SAMPLE_CONCEPTS = SAMPLE_CONCEPTS;
