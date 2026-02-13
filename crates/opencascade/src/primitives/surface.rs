use crate::primitives::make_point;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::geom;
use opencascade_sys::t_colgp;

pub struct Surface {
    pub(crate) inner: UniquePtr<geom::HandleGeomSurface>,
}

impl Surface {
    /// Create a Bezier surface from a 2D grid of control points (poles).
    ///
    /// The outer iterator yields rows (u-direction), each row yields columns (v-direction).
    /// All rows must have the same number of points.
    pub fn bezier(poles: impl IntoIterator<Item = impl IntoIterator<Item = DVec3>>) -> Self {
        let rows: Vec<Vec<DVec3>> = poles.into_iter().map(|r| r.into_iter().collect()).collect();
        let n_rows = rows.len() as i32;
        let n_cols = rows[0].len() as i32;

        let mut array = t_colgp::Array2OfPnt::new_with_bounds(1, n_rows, 1, n_cols);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(
                row.len() as i32, n_cols,
                "All rows must have the same number of control points"
            );
            for (j, p) in row.iter().enumerate() {
                let pnt = make_point(*p);
                array
                    .pin_mut()
                    .set_value(i as i32 + 1, j as i32 + 1, &pnt);
            }
        }

        let surface = geom::BezierSurface::new_array2ofpnt(&array);
        let handle = geom::BezierSurface::to_handle(surface);
        let handle_surface = handle.to_handle_surface();

        Self {
            inner: handle_surface,
        }
    }
}
