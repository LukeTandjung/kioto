// App — top-level shell. Sidebar + active panel + composer + dialogs.

function App() {
  const [tab, setTab] = React.useState("concepts");
  const [dialog, setDialog] = React.useState(null);   // "build" | "add" | null
  const [toast, setToast] = React.useState(null);

  const fragmentOptions = SAMPLE_FRAGMENTS.map(f => ({ id: f.id, label: f.title }));
  const conceptOptions  = SAMPLE_CONCEPTS.map(c  => ({ id: c.id, label: c.title }));

  const fireToast = (text) => {
    setToast(text);
    setTimeout(() => setToast(null), 2200);
  };

  return (
    <div className="app">
      <Sidebar active={tab} onChange={setTab} />
      <main className="app-main">
        {tab === "concepts"  && <ConceptsPanel  onBuild={() => setDialog("build")} />}
        {tab === "fragments" && <FragmentsPanel onAdd={() => setDialog("add")} />}
        {tab === "notes"     && <NotesPanel />}
        <div className="canvas">
          <Wordmark size={20} />
          <EmptyComposer
            fragmentOptions={fragmentOptions}
            conceptOptions={conceptOptions}
            onSend={({ body }) => fireToast(`Sent — “${body.slice(0, 48).trim()}${body.length > 48 ? "…" : ""}”`)}
          />
        </div>
      </main>

      <BuildConceptDialog
        open={dialog === "build"}
        onClose={() => setDialog(null)}
        fragmentOptions={fragmentOptions}
        onSubmit={({ name }) => fireToast(`Built concept “${name}”`)}
      />
      <AddFragmentsDialog
        open={dialog === "add"}
        onClose={() => setDialog(null)}
        onSubmit={({ name }) => fireToast(`Added fragment “${name}”`)}
      />

      {toast && <div className="toast">{toast}</div>}
    </div>
  );
}

window.App = App;
