#[derive(PartialEq,Debug)]
pub struct USD(i32);
#[derive(PartialEq,Debug)]
pub struct GBP(i32);
#[derive(PartialEq,Debug)]
pub struct PLN(i32);

pub trait ToUSDv<F>
{
    fn to_uv(&self,_: F)->f32;
}

pub trait FromUSDv<F>
{
    fn from_uv(&self,_: f32)->F;
}

pub struct Ex
{
    pln:f32,
    gbp:f32,
}

impl ToUSDv<GBP> for Ex
{
    fn to_uv(&self, g:GBP)->f32
    {
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<PLN> for Ex
{
    fn from_uv(&self, f:f32)->PLN
    {
        PLN((f /self.pln) as i32)
    }
}

pub trait Exchange<F,T>
{
    fn convert(&self,_: F)->T;
}

impl<E,F,T> Exchange<F,T> for E
where E:ToUSDv<F> + FromUSDv<T>
{
  fn convert(&self, f:F)->T
  {
      self.from_uv(self.to_uv(f))
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let g1 = GBP(200);
        let ex = Ex{pln:0.7, gbp: 1.3};
        let c = ex.from_uv(ex.to_uv(g));
        let c2:PLN = ex.convert(g1);
        assert_eq!(c, PLN(371));
        assert_eq!(c2, PLN(371));
    }
}
