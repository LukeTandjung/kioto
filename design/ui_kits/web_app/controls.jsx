// Form primitives — Button, TextField, Textarea, Search, Checkbox.
// All on-brand by default. Pure cosmetic — no validation.

function Button({ children, variant = "primary", leadingIcon, size = "md", disabled, onClick, type = "button" }) {
  const cls = ["btn", `btn-${variant}`, size === "lg" && "btn-lg"].filter(Boolean).join(" ");
  return (
    <button type={type} className={cls} disabled={disabled} onClick={onClick}>
      {leadingIcon && <span className="btn-icon">{leadingIcon}</span>}
      {children}
    </button>
  );
}

function TextField({ value, onChange, placeholder, autoFocus }) {
  return (
    <input
      className="textfield"
      value={value || ""}
      onChange={onChange ? (e) => onChange(e.target.value) : undefined}
      placeholder={placeholder}
      autoFocus={autoFocus}
    />
  );
}

function Textarea({ value, onChange, placeholder, rows = 4 }) {
  return (
    <textarea
      className="textarea"
      rows={rows}
      value={value || ""}
      onChange={onChange ? (e) => onChange(e.target.value) : undefined}
      placeholder={placeholder}
    />
  );
}

function Search({ value, onChange, placeholder = "Search..." }) {
  return (
    <div className="search">
      <span className="search-icon"><Icon.Search className="ico-12" /></span>
      <input
        className="search-input"
        value={value || ""}
        onChange={onChange ? (e) => onChange(e.target.value) : undefined}
        placeholder={placeholder}
      />
    </div>
  );
}

function Checkbox({ checked, onChange, label }) {
  return (
    <label className="cbx-row">
      <button
        type="button"
        className={`cbx ${checked ? "cbx-on" : ""}`}
        onClick={() => onChange && onChange(!checked)}
        aria-pressed={checked}
      >
        {checked && <Icon.Check className="ico-10" />}
      </button>
      {label && <span className="cbx-label">{label}</span>}
    </label>
  );
}

function Field({ label, help, children }) {
  return (
    <div className="field">
      <div className="field-head">
        <div className="field-label">{label}</div>
        {help && <div className="field-help">{help}</div>}
      </div>
      {children}
    </div>
  );
}

Object.assign(window, { Button, TextField, Textarea, Search, Checkbox, Field });
