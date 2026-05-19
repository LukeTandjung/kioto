// Sidebar — fixed 60px rail. Three tabs at top, cog + avatar at bottom.

function Sidebar({ active, onChange }) {
  const tabs = [
    { id: "notes",     label: "Notes",     Glyph: Icon.BookOpen },
    { id: "concepts",  label: "Concepts",  Glyph: Icon.LightBulb },
    { id: "fragments", label: "Fragments", Glyph: Icon.Photo },
  ];
  return (
    <aside className="sidebar">
      <div className="sidebar-tabs">
        {tabs.map(({ id, label, Glyph }) => (
          <button
            key={id}
            className={`tab ${active === id ? "tab-active" : ""}`}
            onClick={() => onChange(id)}
            title={label}
          >
            <Glyph className="ico-22" />
          </button>
        ))}
      </div>
      <div className="sidebar-bottom">
        <button className="cog" title="Settings"><Icon.Cog className="ico-22" /></button>
        <div className="avatar" style={{ backgroundImage: "url('../../assets/avatar-sample.jpg')" }} />
      </div>
    </aside>
  );
}

window.Sidebar = Sidebar;
