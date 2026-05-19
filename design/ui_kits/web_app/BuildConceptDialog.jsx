// Build Concept dialog — Name + Fragment select + Description.
function BuildConceptDialog({ open, onClose, fragmentOptions, onSubmit }) {
  const [name, setName] = React.useState("");
  const [fragment, setFragment] = React.useState(fragmentOptions[0]?.id);
  const [description, setDescription] = React.useState("");

  const reset = () => { setName(""); setFragment(fragmentOptions[0]?.id); setDescription(""); };
  const close = () => { reset(); onClose && onClose(); };
  const submit = () => { onSubmit && onSubmit({ name, fragment, description }); close(); };

  return (
    <Dialog
      open={open}
      onClose={close}
      title="Build Concept"
      help="Select the fragments your concept will be built from."
      footer={(
        <>
          <Button variant="secondary" onClick={close}>Cancel</Button>
          <Button variant="primary" onClick={submit} disabled={!name.trim()}>Build Concept</Button>
        </>
      )}
    >
      <div className="dialog-grid">
        <Field label="Name">
          <TextField value={name} onChange={setName} placeholder="Untitled" autoFocus />
        </Field>
        <Field label="Fragment">
          <div className="select">
            <select value={fragment} onChange={(e) => setFragment(e.target.value)}>
              {fragmentOptions.map(o => <option key={o.id} value={o.id}>{o.label}</option>)}
            </select>
            <Icon.ChevronDown className="ico-12 ico-base select-arrow" />
          </div>
        </Field>
      </div>
      <Field label="Description" help="Describe the concept you want to extract from the fragment.">
        <Textarea value={description} onChange={setDescription} rows={3} />
      </Field>
    </Dialog>
  );
}

window.BuildConceptDialog = BuildConceptDialog;
