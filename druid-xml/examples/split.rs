
fn main() {
	druid_xml!(
	"<split direction=column number=2>
		<label>Left content</label>
		<split direction=row number=2>
			<label>Right upper</label>
			<label>Right down</label>
		</split>
	</split>",
	
	"label" => |e| {
		e.mouse_over( || {
			e.background( Color::white );
		});
	});
}