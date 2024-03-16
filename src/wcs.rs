use mapproj::{
    img2proj::WcsImgXY2ProjXY,
    img2celestial::Img2Celestial,
    ImgXY, CenteredProjection, LonLat,
    cylindrical::car::Car,
};

#[allow(dead_code)]
pub struct WCS {
    crpix1: f64,
    crpix2: f64,
    cdelt1: f64,
    cdelt2: f64,
    crval1: f64,
    crval2: f64,
    wcs: Img2Celestial<Car, WcsImgXY2ProjXY>
}

impl WCS {

    #[allow(dead_code)]
    pub fn new(crpix1: f64, crpix2: f64, cdelt1: f64, cdelt2: f64, crval1: f64, crval2: f64) -> WCS {
        let mut centered_proj = CenteredProjection::new(Car::default());
        let proj_center = LonLat::new(crval1.to_radians(), crval2.to_radians());
        centered_proj.set_proj_center_from_lonlat(&proj_center);
        let img2proj = WcsImgXY2ProjXY::from_cr(crpix1, crpix2, 0.0, cdelt1, cdelt2);
        let wcs = Img2Celestial::new(img2proj, centered_proj);
        WCS {
            crpix1,
            crpix2,
            cdelt1,
            cdelt2,
            crval1,
            crval2,
            wcs,
        }
    }

    #[allow(dead_code)]
    pub fn pix2sky(&self, x: f64, y: f64) -> (f64, f64) {
        let xy = ImgXY::new(x+1., y+1.);  // mapproj convention is coord starts from 1
        let lonlat = self.wcs.img2lonlat(&xy).unwrap();
        (lonlat.lon(), lonlat.lat())
    }

    #[allow(dead_code)]
    pub fn sky2pix(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lonlat = LonLat::new(lon, lat);
        let xy = self.wcs.lonlat2img(&lonlat).unwrap();
        (xy.x()-1., xy.y()-1.)  // mapproj convention is coord starts from 1, undo it now
    }

    // getters so that they don't get overwritten
    pub fn cdelt1(&self) -> f64 { self.cdelt1 }
    pub fn cdelt2(&self) -> f64 { self.cdelt2 }
    pub fn crval1(&self) -> f64 { self.crval1 }
    pub fn crval2(&self) -> f64 { self.crval2 }
    pub fn crpix1(&self) -> f64 { self.crpix1 }
    pub fn crpix2(&self) -> f64 { self.crpix2 }
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_pix2sky_conversion() {
        let wcs = WCS::new(180.5, 91., -1., 1., 0.5, 0.0);

        let (lon, lat) = wcs.pix2sky(0., 0.);

        // Assert that the actual and expected values are close enough
        assert_relative_eq!(lon, 180_f64.to_radians(), epsilon = 1e-6);
        assert_relative_eq!(lat, -90_f64.to_radians(), epsilon = 1e-6);

        let (lon, lat) = wcs.pix2sky(22., 11.5);

        // Assert that the actual and expected values are close enough
        assert_relative_eq!(lon, 158_f64.to_radians(), epsilon = 1e-6);
        assert_relative_eq!(lat, -78.5_f64.to_radians(), epsilon = 1e-6);
    }

    #[test]
    fn test_sky2pix_conversion() {
        let wcs = WCS::new(180.5, 91., -1., 1., 0.5, 0.0);

        let (x, y) = wcs.sky2pix(0., 0.);

        assert_relative_eq!(x, 180., epsilon = 1e-6);
        assert_relative_eq!(y, 90., epsilon = 1e-6);
    }
}