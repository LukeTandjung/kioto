// Wordmark — the canonical {Concise.} lockup.
// Reads font from --font-wordmark (Ranade) with a DM Sans fallback.
// The dot is the only spot of accent; braces stay dimmed.

function Wordmark({ size = 20, withBraces = true }) {
  const style = { fontSize: size, lineHeight: 1 };
  return (
    <div className="wordmark" style={style}>
      {withBraces && <span className="wm-brace">{"{"}</span>}
      <span className="wm-name">Concise<span className="wm-dot">.</span></span>
      {withBraces && <span className="wm-brace">{"}"}</span>}
    </div>
  );
}

window.Wordmark = Wordmark;
