// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>

#![cfg_attr(not(feature = "std"), no_std)]

use staging_xcm::latest::prelude::*;
pub trait Parse {
    /// Returns the "chain" location part. It could be parent, sibling
    /// parachain, or child parachain.
    fn chain_part(&self) -> Option<Location>;
    /// Returns "non-chain" location part.
    fn non_chain_part(&self) -> Option<Location>;
}

impl Parse for Location {
    fn chain_part(&self) -> Option<Location> {
        match (self.parents, self.first_interior()) {
            // sibling parachain
            (1, Some(Parachain(id))) => Some(Location::new(1, [Parachain(*id)])),
            // parent
            (1, _) => Some(Location::parent()),
            // children parachain
            (0, Some(Parachain(id))) => Some(Location::new(0, [Parachain(*id)])),
            _ => None,
        }
    }

    fn non_chain_part(&self) -> Option<Location> {
        let mut junctions = self.interior().clone();
        while matches!(junctions.first(), Some(Parachain(_))) {
            let _ = junctions.take_first();
        }

        if junctions != Here {
            Some(Location::new(0, junctions))
        } else {
            None
        }
    }
}
