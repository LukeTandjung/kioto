// NotesPanel — outline of nested headings.
const SAMPLE_NOTES = [
  { title: "Real Analysis", children: [
    { title: "Real Number System", children: [
      { title: "Basic Axioms" },
      { title: "Suprema & Infima" },
      { title: "Completeness" },
    ]},
    { title: "Sequences and Conver…", children: [
      { title: "Limit Definitions", children: [
        { title: "Pointwise Converg…" },
        { title: "Uniform Converge…" },
      ]},
      { title: "Cauchy Sequences" },
      { title: "Compactness" },
    ]},
  ]},
  { title: "Complex Analysis", children: [
    { title: "Complex Number System" },
    { title: "Complex Algebra" },
    { title: "Different Forms", children: [
      { title: "Plane Form" },
      { title: "Polar Form" },
      { title: "Exponential Form" },
    ]},
    { title: "Complex Functions", children: [
      { title: "Complex Mappings" },
      { title: "Limits" },
      { title: "Continuity" },
      { title: "Holomorphic" },
      { title: "Cauchy Riemann Equ…" },
    ]},
  ]},
];

function NotesPanel() {
  const [search, setSearch] = React.useState("");
  return (
    <Panel title="Notes" search={search} onSearch={setSearch}>
      {SAMPLE_NOTES.map((n, i) => <NoteNode key={i} node={n} />)}
    </Panel>
  );
}

window.NotesPanel = NotesPanel;
window.SAMPLE_NOTES = SAMPLE_NOTES;
