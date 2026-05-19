// MultiSelect — Headless-UI-style trigger + checkbox-list popover.
// Click outside or press Esc to close.
function MultiSelect({ label, options, value = [], onChange }) {
  const [open, setOpen] = React.useState(false);
  const [search, setSearch] = React.useState("");
  const wrapRef = React.useRef(null);

  React.useEffect(() => {
    if (!open) return;
    const onDoc = (e) => { if (!wrapRef.current?.contains(e.target)) setOpen(false); };
    const onKey = (e) => { if (e.key === "Escape") setOpen(false); };
    document.addEventListener("mousedown", onDoc);
    document.addEventListener("keydown", onKey);
    return () => {
      document.removeEventListener("mousedown", onDoc);
      document.removeEventListener("keydown", onKey);
    };
  }, [open]);

  const toggle = (id) => {
    const next = value.includes(id) ? value.filter(x => x !== id) : [...value, id];
    onChange && onChange(next);
  };

  const display = value.length === 0
    ? label
    : value.length === 1
      ? options.find(o => o.id === value[0])?.label
      : `${label} · ${value.length}`;

  const filtered = options.filter(o => o.label.toLowerCase().includes(search.toLowerCase()));

  return (
    <div className="ms" ref={wrapRef}>
      <button type="button" className={`ms-trigger ${value.length ? "ms-trigger-on" : ""}`} onClick={() => setOpen(o => !o)}>
        <span className="ms-value">{display}</span>
        <Icon.ChevronDown className="ico-12 ico-base" />
      </button>
      {open && (
        <div className="ms-popover" role="listbox">
          <div className="ms-header">
            <Checkbox
              checked={value.length === options.length}
              onChange={(on) => onChange && onChange(on ? options.map(o => o.id) : [])}
            />
            <div className="ms-search">
              <Icon.Search className="ico-12 ico-mute" />
              <input
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                placeholder="Search..."
              />
            </div>
          </div>
          <div className="ms-divider" />
          <ul className="ms-list">
            {filtered.map(o => (
              <li key={o.id}>
                <Checkbox
                  checked={value.includes(o.id)}
                  onChange={() => toggle(o.id)}
                  label={o.label}
                />
              </li>
            ))}
            {filtered.length === 0 && <li className="ms-empty">No matches</li>}
          </ul>
        </div>
      )}
    </div>
  );
}

window.MultiSelect = MultiSelect;
