// Copyright 2019-2020 promethium Protocol.
// This file is part of promethium.

// promethium is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with promethium.  If not, see <http://www.gnu.org/licenses/>.

//! promethium Node CLI

#![warn(missing_docs)]

fn main() -> sc_cli::Result<()> {
	node_cli::run()
}
