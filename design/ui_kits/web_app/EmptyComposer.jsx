// EmptyComposer — the centred 514px prompt + textarea + multi-selects + send.
function EmptyComposer({ onSend, fragmentOptions, conceptOptions }) {
  const [body, setBody] = React.useState(
    "I want to learn more about limits, including the different types of limits in the respective metric, norm, and inner product space. Break it down into the following sections:"
  );
  const [fragments, setFragments] = React.useState([]);
  const [concepts, setConcepts]   = React.useState([]);

  return (
    <section className="empty-composer">
      <div className="prompt">What do you want to learn today?</div>
      <div className="composer">
        <textarea
          className="composer-body"
          value={body}
          onChange={(e) => setBody(e.target.value)}
          rows={3}
        />
        <div className="composer-footer">
          <div className="composer-chips">
            <MultiSelect
              label="Select Fragments"
              options={fragmentOptions}
              value={fragments}
              onChange={setFragments}
            />
            <MultiSelect
              label="Select Concepts"
              options={conceptOptions}
              value={concepts}
              onChange={setConcepts}
            />
          </div>
          <button
            className="send"
            disabled={!body.trim()}
            onClick={() => onSend && onSend({ body, fragments, concepts })}
            title="Send"
          >
            <Icon.PaperAirplane className="ico-16" />
          </button>
        </div>
      </div>
    </section>
  );
}

window.EmptyComposer = EmptyComposer;
