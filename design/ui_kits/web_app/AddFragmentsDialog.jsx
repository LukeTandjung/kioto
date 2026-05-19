// Add Fragments dialog — Name + Description + Upload dropzone.
function AddFragmentsDialog({ open, onClose, onSubmit }) {
  const [name, setName] = React.useState("");
  const [description, setDescription] = React.useState("");
  const [file, setFile] = React.useState(null);

  const reset = () => { setName(""); setDescription(""); setFile(null); };
  const close = () => { reset(); onClose && onClose(); };
  const submit = () => { onSubmit && onSubmit({ name, description, file }); close(); };

  return (
    <Dialog
      open={open}
      onClose={close}
      title="Add Fragments"
      help="Upload a folder with files to process into fragments."
      footer={(
        <>
          <Button variant="secondary" onClick={close}>Cancel</Button>
          <Button variant="primary" onClick={submit} disabled={!name.trim()}>Add Fragment</Button>
        </>
      )}
    >
      <Field label="Name">
        <TextField value={name} onChange={setName} placeholder="Lecture 4" autoFocus />
      </Field>
      <Field label="Description" help="Describe what this fragment is.">
        <Textarea value={description} onChange={setDescription} rows={3} />
      </Field>
      <Field label="Upload Folder">
        <label className={`dropzone ${file ? "dropzone-on" : ""}`}>
          <input
            type="file"
            multiple
            onChange={(e) => setFile(e.target.files?.length ? `${e.target.files.length} file${e.target.files.length === 1 ? "" : "s"}` : null)}
          />
          <div className="dropzone-text">{file || "Upload or browse"}</div>
          <Icon.ArrowUpTray className="ico-16 ico-base" />
        </label>
      </Field>
    </Dialog>
  );
}

window.AddFragmentsDialog = AddFragmentsDialog;
