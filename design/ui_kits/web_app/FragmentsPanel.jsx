// FragmentsPanel — Fragment Store contents.
const SAMPLE_FRAGMENTS = [
  {
    id: "lec1", title: "Lecture 1", files: [
      { kind: "pdf",   name: "week_one_slides.pdf" },
      { kind: "image", name: "board_screen.png"    },
      { kind: "video", name: "live_tutorial_cast.mp4" },
    ],
  },
  {
    id: "lec2", title: "Lecture 2", files: [
      { kind: "pdf",   name: "week_two_slides.pdf" },
      { kind: "image", name: "board_screen.png"    },
      { kind: "video", name: "live_tutorial_cast.mp4" },
    ],
  },
  {
    id: "lec3", title: "Lecture 3", files: [
      { kind: "pdf",   name: "week_three_slides.pdf" },
      { kind: "image", name: "board_screen.png"      },
      { kind: "video", name: "live_tutorial_cast.mp4" },
    ],
  },
];

function FragmentsPanel({ onAdd }) {
  const [search, setSearch] = React.useState("");
  const items = SAMPLE_FRAGMENTS.filter(f =>
    f.title.toLowerCase().includes(search.toLowerCase()) ||
    f.files.some(x => x.name.toLowerCase().includes(search.toLowerCase()))
  );
  return (
    <Panel
      title="Fragment Store"
      search={search}
      onSearch={setSearch}
      footer={
        <Button variant="primary" leadingIcon={<Icon.Plus className="ico-12" />} onClick={onAdd}>
          Add Fragments
        </Button>
      }
    >
      {items.map(f => (
        <FragmentRow key={f.id} title={f.title} files={f.files} />
      ))}
    </Panel>
  );
}

window.FragmentsPanel = FragmentsPanel;
window.SAMPLE_FRAGMENTS = SAMPLE_FRAGMENTS;
