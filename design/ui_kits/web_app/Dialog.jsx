// Dialog shell — fixed scrim + centred 560px card.
// Press Esc to close.

function Dialog({ open, onClose, title, help, children, footer }) {
  React.useEffect(() => {
    if (!open) return;
    const onKey = (e) => { if (e.key === "Escape") onClose && onClose(); };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  if (!open) return null;
  return (
    <div className="dialog-scrim" onMouseDown={onClose}>
      <div className="dialog" onMouseDown={(e) => e.stopPropagation()} role="dialog" aria-modal="true">
        <header className="dialog-head">
          <div className="dialog-title">{title}</div>
          {help && <div className="dialog-help">{help}</div>}
        </header>
        <div className="dialog-body">{children}</div>
        {footer && <footer className="dialog-actions">{footer}</footer>}
      </div>
    </div>
  );
}

window.Dialog = Dialog;
