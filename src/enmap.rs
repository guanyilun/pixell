use ndarray::prelude::*;
use crate::wcs::WCS;

type Shape = (usize, usize);

// res: deg
pub fn fullsky_geometry(res: f64) -> (Shape, WCS) {
    let nx = (360.0 / res).round() as usize;
    let ny = (180.0 / res).round() as usize + 1;
    let crval = [res/2.0, 0.];
    let cdelt = [-360.0 / (nx as f64), 180.0 / (ny-1) as f64];
    let crpix = [(nx / 2) as f64 + 0.5, (ny+1) as f64 / 2.0];
    let wcs = WCS::new(crpix[0], crpix[1], cdelt[0], cdelt[1], crval[0], crval[1]);
    let shape = (nx, ny);
    (shape, wcs)
}

// res, dec_cut: deg, deg
pub fn band_geometry(res: f64, dec_cut: f64) -> (Shape, WCS) {
    let (shape, wcs_fullsky) = fullsky_geometry(res);
    let (dec_min, dec_max) = (-dec_cut, dec_cut);
    let (_, start) = wcs_fullsky.sky2pix(0., dec_min.to_radians());
    let (_, stop) = wcs_fullsky.sky2pix(0., dec_max.to_radians());
    let start = (start.round() as usize).max(0_usize);
    let stop = (stop.round() as usize).min(shape.1);
    let crpix2 = wcs_fullsky.crpix2() - start as f64;
    let ny_new = stop - start;
    let wcs = WCS::new(wcs_fullsky.crpix1(), crpix2, wcs_fullsky.cdelt1(), wcs_fullsky.cdelt2(), wcs_fullsky.crval1(), wcs_fullsky.crval2());

    ((shape.0, ny_new), wcs)
}

#[allow(dead_code)]
pub struct Enmap {
    pub data: Array2<f64>,
    pub wcs: WCS,
}

#[allow(dead_code)]
pub fn zeros(shape: Shape, wcs: WCS) -> Enmap {
    let data = Array::<f64, _>::zeros(shape.f());  // column order
    Enmap {
        data,
        wcs,
    }
}

impl Enmap {
    pub fn posmap(&self) -> (Array2<f64>, Array2<f64>) {
        let (nx, ny) = self.data.dim();
        let mut x = Array2::<f64>::zeros((nx, ny).f());
        let mut y = Array2::<f64>::zeros((nx, ny).f());
        for i in 0..nx {
            for j in 0..ny {
                let (lon, lat) = self.wcs.pix2sky(i as f64, j as f64);
                x[[i, j]] = lon;
                y[[i, j]] = lat;
            }
        }
        (x, y)
    }
}


#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_band_geometry() {
        let res = 1.0;
        let dec_cut = 30.0;
        let ((nx, ny), wcs) = band_geometry(res, dec_cut);
        assert_eq!(nx, 360);
        assert_eq!(ny, 60);
        assert_eq!(wcs.crpix1(), 180.5);
        assert_eq!(wcs.crpix2(), 31.0);
        assert_eq!(wcs.cdelt1(), -1.0);
        assert_eq!(wcs.cdelt2(), 1.0);
        assert_eq!(wcs.crval1(), 0.5);
        assert_eq!(wcs.crval2(), 0.0);
        assert_eq!(wcs.sky2pix(0.0, 0.0), (180., 30.));
    }

    #[test]
    fn test_fullsky_geometry() {
        let res = 1.0;
        let ((nx, ny), wcs) = fullsky_geometry(res);
        assert_eq!(nx, 360);
        assert_eq!(ny, 181);
        assert_eq!(wcs.crpix1(), 180.5);
        assert_eq!(wcs.crpix2(), 91.0);
        assert_eq!(wcs.cdelt1(), -1.0);
        assert_eq!(wcs.cdelt2(), 1.0);
        assert_eq!(wcs.crval1(), 0.5);
        assert_eq!(wcs.crval2(), 0.0);
        assert_eq!(wcs.sky2pix(0.0, 0.0), (180., 90.));
    }

    #[test]
    fn test_enmap() {
        let (shape, wcs) = fullsky_geometry(1.0);
        let enmap = zeros(shape, wcs);
        assert_eq!(enmap.data.sum(), 0.);

        let (x, y) = enmap.posmap();
        assert_relative_eq!(x[[13,11]], 2.914700, epsilon = 1e-6);
        assert_relative_eq!(y[[12,10]], -1.396263, epsilon = 1e-6);
    }
}