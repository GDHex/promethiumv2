// Copyright 2019-2020 promethium Protocol.
// This file is part of promethium.

// promethium is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with promethium.  If not, see <http://www.gnu.org/licenses/>.

use wasm_builder_runner::WasmBuilder;

fn main() {
	WasmBuilder::new()
		.with_current_project()
		.with_wasm_builder_from_crates_or_path("2.0.0", "../../utils/wasm-builder")
		.export_heap_base()
		.import_memory()
		.build()
}
