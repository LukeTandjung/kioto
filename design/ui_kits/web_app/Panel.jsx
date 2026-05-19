// Generic 180px panel chrome — title, search, scrollable body, optional CTA.

function Panel({ title, search, onSearch, children, footer }) {
  return (
    <section className="panel">
      <header className="panel-head">
        <div className="panel-title">{title}</div>
        <Search value={search} onChange={onSearch} />
      </header>
      <div className="panel-body">{children}</div>
      {footer && <footer className="panel-footer">{footer}</footer>}
    </section>
  );
}

// A single concept row: leading question-mark icon, title, then sub-items.
function ConceptRow({ title, subitems = [], onClick, active }) {
  return (
    <div className={`concept-row ${active ? "concept-row-active" : ""}`} onClick={onClick}>
      <div className="concept-title">
        <Icon.QuestionMarkCircle className="ico-12 ico-mute" />
        <span className="truncate">{title}</span>
      </div>
      {subitems.length > 0 && (
        <ul className="concept-subs">
          {subitems.map((s, i) => <li key={i} className="truncate">{s}</li>)}
        </ul>
      )}
    </div>
  );
}

// A fragment-folder row: leading folder icon, title, then file children.
function FragmentRow({ title, files = [] }) {
  const fileIcon = (kind) => {
    if (kind === "pdf")   return <Icon.DocumentText className="ico-12 ico-mute" />;
    if (kind === "image") return <Icon.PhotoMini    className="ico-12 ico-mute" />;
    if (kind === "video") return <Icon.PlayCircle   className="ico-12 ico-mute" />;
    return <Icon.DocumentText className="ico-12 ico-mute" />;
  };
  return (
    <div className="fragment-row">
      <div className="fragment-title">
        <Icon.Folder className="ico-12" />
        <span className="truncate">{title}</span>
      </div>
      <ul className="fragment-files">
        {files.map((f, i) => (
          <li key={i}>{fileIcon(f.kind)}<span className="truncate">{f.name}</span></li>
        ))}
      </ul>
    </div>
  );
}

// A note tree row — used recursively.
function NoteNode({ node, depth = 0 }) {
  return (
    <div className="note-node" style={{ paddingLeft: depth === 0 ? 0 : 12 }}>
      <div className={depth === 0 ? "note-title" : "note-sub"}>
        {depth === 0 && <Icon.DocumentText className="ico-12" />}
        <span className="truncate">{node.title}</span>
      </div>
      {node.children?.map((c, i) => <NoteNode key={i} node={c} depth={depth + 1} />)}
    </div>
  );
}

Object.assign(window, { Panel, ConceptRow, FragmentRow, NoteNode });
