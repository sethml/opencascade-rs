use crate::primitives::make_point;
use glam::DVec3;
use opencascade_sys::{geom, n_collection};

pub struct Surface {
    pub(crate) inner: opencascade_sys::OwnedPtr<geom::Handle_Geom_Surface>,
}

impl Surface {
    pub fn bezier(poles: impl IntoIterator<Item = impl IntoIterator<Item = DVec3>>) -> Self {
        let poles: Vec<Vec<_>> =
            poles.into_iter().map(|poles| poles.into_iter().collect()).collect();

        let mut pole_array = n_collection::Array2_gp_Pnt::new_int4(
            0,
            poles.len() as i32 - 1,
            0,
            poles.first().map(|first| first.len()).unwrap_or(0) as i32 - 1,
        );

        for (row, poles) in poles.iter().enumerate() {
            for (column, pole) in poles.iter().enumerate() {
                let pole = &make_point(*pole);
                pole_array.set_value(row as i32, column as i32, pole);
            }
        }

        let bezier = geom::BezierSurface::new_arr2_pnt(&pole_array);
        let handle = geom::BezierSurface::to_handle(bezier);
        let inner = handle.to_handle_surface();

        Self { inner }
    }
}
