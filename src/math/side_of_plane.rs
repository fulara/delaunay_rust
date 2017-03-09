use types::Point3;
use types::Point3Err;
use types::point3_err_from_point3;
use math::F64Err;
use cgmath::Matrix3;
use cgmath::SquareMatrix;
use math::order_float;
use types::fp;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum SideOfPlane {
    Left,
    Right,
    OnPlane,
}

#[inline]
pub fn side_of_plane(a: &Point3, b: &Point3, c: &Point3, p: &Point3) -> SideOfPlane {
    let a = point3_err_from_point3(&a);
    let b = point3_err_from_point3(&b);
    let c = point3_err_from_point3(&c);
    let p = point3_err_from_point3(&p);

    let ab = b - a;
    let ac = c - a;
    let ap = p - a;

    let det = Matrix3::new(ab.x, ab.y, ab.z,
                           ac.x, ac.y, ac.z,
                           ap.x, ap.y, ap.z).determinant();

   /* let arr_x = [ab.x, ac.x, ap.x];
    let arr_y = [ab.y, ac.y, ap.y];
    let arr_z = [ab.z, ac.z, ap.z];

    let max_val_x = arr_x.iter().max_by(order_float).unwrap();
    let min_val_x = arr_x.iter().min_by(order_float).unwrap();

    let max_val_y = arr_y.iter().max_by(order_float).unwrap();
    let min_val_y = arr_y.iter().min_by(order_float).unwrap();

    let max_val_z = arr_z.iter().max_by(order_float).unwrap();
    let min_val_z = arr_z.iter().min_by(order_float).unwrap();


    let max_x = max_val_x.max(min_val_x.abs());
    let max_y = max_val_y.max(min_val_y.abs());
    let max_z = max_val_z.max(min_val_z.abs());

    let eux = b.x.abs().max(a.x.abs());
    let euy = b.y.abs().max(a.y.abs());
    let euz = b.z.abs().max(a.z.abs());

    let evx = c.x.abs().max(a.x.abs());
    let evy = c.y.abs().max(a.y.abs());
    let evz = c.z.abs().max(a.z.abs());

    let ewx = p.x.abs().max(a.x.abs());
    let ewy = p.y.abs().max(a.y.abs());
    let ewz = p.z.abs().max(a.z.abs());

    //self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
    //self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
    //self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])

    let e = eux*((ac.y*ap.z).abs() + (ac.z*ap.y).abs()) + euy*((ac.z*ap.x).abs() + (ac.x*ap.z).abs()) + euz*((ac.x*ap.y).abs() + (ac.y*ap.x).abs())
        + evx*((ab.y*ap.z).abs() + (ab.z*ap.y).abs()) + evy*((ab.z*ap.x).abs() + (ab.x*ap.z).abs()) + evz*((ab.x*ap.y).abs() + (ab.y*ap.x).abs())
        + ewx*((ab.y*ac.z).abs() + (ab.z*ac.y).abs()) + ewy*((ab.z*ac.x).abs() + (ab.x*ac.z).abs()) + ewz*((ab.x*ac.y).abs() + (ab.y*ac.x).abs());

    let eps = e.abs() * fp::EPSILON;

    //println!("det is: {:?} a {:?} b {:?} c {:?} p {:?} eps is: {:?} e1 {} e2 {} e3 {} e4 {}", det, a,b,c,p, eps, e1, e2, e3, e4);
*/
    return SideOfPlane::Right;
    /*if det < -eps {
        SideOfPlane::Right
    } else if det > eps {
        SideOfPlane::Left
    } else {
        SideOfPlane::OnPlane
    }*/
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::Point3;

    #[test]
    fn testing_simplest_plane() {
        let a = Point3::new(0., 0., 0.);
        let b = Point3::new(0., 1., 0.);
        let c = Point3::new(1., 0., 0.);

        assert_eq!(SideOfPlane::OnPlane, side_of_plane(&a, &b, &c, &Point3::new(0.5, 0.5, 0.)));
        assert_eq!(SideOfPlane::OnPlane, side_of_plane(&a, &b, &c, &Point3::new(0.5, -5000., 0.)));
        assert_eq!(SideOfPlane::OnPlane, side_of_plane(&a, &b, &c, &Point3::new(500., -2000., 0.)));

        assert_eq!(SideOfPlane::Left, side_of_plane(&a, &b, &c, &Point3::new(0., 0., -1.)));
        assert_eq!(SideOfPlane::Left, side_of_plane(&a, &b, &c, &Point3::new(50., 50., -1.)));

        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &Point3::new(0., 0., 1.)));
        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &Point3::new(50., 50., 1.)));
    }

    #[test]
    fn testing_random_plane() {
        let a = Point3::new(5., 10., 12.);
        let b = Point3::new(8., 10., 11.);
        let c = Point3::new(3., 3., 2.);

        let center = Point3::new((a.x + b.x + c.x) / 3., (a.y + b.y + c.y) / 3., (a.z + b.z + c.z) / 3.);

        assert_eq!(SideOfPlane::OnPlane, side_of_plane(&a, &b, &c, &center));
    }

    #[test]
    fn testing_ordering() {
        let a = Point3::new(0., 0., 0.);
        let b = Point3::new(0., 1., 0.);
        let c = Point3::new(1., 0., 0.);
        let d = Point3::new(1., 1., 1.);

        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &d));
        assert_eq!(SideOfPlane::Right, side_of_plane(&b, &a, &d, &c));
        assert_eq!(SideOfPlane::Right, side_of_plane(&d, &c, &b, &a));
        assert_eq!(SideOfPlane::Right, side_of_plane(&d, &b, &a, &c));
    }

    #[test]
    fn testing_center_should_be_on_the_same_side() {
        let a = Point3::new(0., 0., 0.);
        let b = Point3::new(0., 1., 0.);
        let c = Point3::new(1., 0., 0.);
        let d = Point3::new(1., 1., 1.);

        let center = Point3::new((a.x + b.x + c.x + d.x) / 4., (a.y + b.y + c.y + d.y) / 4., (a.z + b.z + c.z + d.z) / 4.);

        //just to make sure.
        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &d));

        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &center));
        assert_eq!(SideOfPlane::Right, side_of_plane(&b, &a, &d, &center));
        assert_eq!(SideOfPlane::Right, side_of_plane(&d, &c, &b, &center));
        assert_eq!(SideOfPlane::Right, side_of_plane(&d, &b, &a, &center));
    }

    #[test]
    fn testing_center_should_be_on_the_same_side_2() {
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(0.0, 0.0, 1.0);
        let c = Point3::new(0.0, 1.0, 0.0);
        let d = Point3::new(1.0, 0.0, 0.0);

        let center = Point3::new((a.x + b.x + c.x + d.x) / 4., (a.y + b.y + c.y + d.y) / 4., (a.z + b.z + c.z + d.z) / 4.);

        //just to make sure.
        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &d));

        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &center));
        assert_eq!(SideOfPlane::Right, side_of_plane(&b, &a, &d, &center));
        assert_eq!(SideOfPlane::Right, side_of_plane(&d, &c, &b, &center));
        assert_eq!(SideOfPlane::Right, side_of_plane(&d, &b, &a, &center));
    }

    #[quickcheck]
    fn quickcheck_test(a_pos: (f64, f64, f64), b_pos: (f64, f64, f64), c_pos: (f64, f64, f64), d_pos: (f64, f64, f64)) {
        if a_pos == b_pos || a_pos == c_pos || a_pos == d_pos || b_pos == c_pos || b_pos == d_pos || c_pos == d_pos {
            return;
        }

        let a = Point3::new(a_pos.0, a_pos.1, a_pos.2);
        let mut b = Point3::new(b_pos.0, b_pos.1, b_pos.2);
        let c = Point3::new(c_pos.0, c_pos.1, c_pos.2);
        let mut d = Point3::new(d_pos.0, d_pos.1, d_pos.2);

        if SideOfPlane::Left == side_of_plane(&a, &b, &c, &d) {
            ::std::mem::swap(&mut b, &mut d);
        } else if SideOfPlane::OnPlane == side_of_plane(&a, &b, &c, &d) {
            return;
        }

        let sides = [(a, b, c), (b, a, d), (d, c, b), (d, b, a)];

        assert_eq!(SideOfPlane::Right, side_of_plane(&a, &b, &c, &d));
        let center = Point3::new((a.x + b.x + c.x + d.x) / 4., (a.y + b.y + c.y + d.y) / 4., (a.z + b.z + c.z + d.z) / 4.);

        for side in sides.iter() {
            let side_center = Point3::new((side.0.x + side.2.x + side.1.x)/3., (side.0.y + side.2.y + side.1.y)/3., (side.0.z + side.2.z + side.1.z)/3.);
            let vec = side_center - center;
            let on_the_other_side = center + (2.* vec);

            let result = side_of_plane(&side.0, &side.1, &side.2, &side_center);
            if result != SideOfPlane::OnPlane {
                panic!(format!("Expected p: {:?} to be on plane of {:?} {:?} {:?}", &side_center, &side.0, &side.1, &side.2));
            }

            assert_eq!(SideOfPlane::Left, side_of_plane(&side.0, &side.1, &side.2, &on_the_other_side));
            assert_eq!(SideOfPlane::Right, side_of_plane(&side.0, &side.1, &side.2, &center))
        }
    }
}