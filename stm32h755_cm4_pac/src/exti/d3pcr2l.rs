#[doc = "Register `D3PCR2L` reader"]
pub type R = crate::R<D3pcr2lSpec>;
#[doc = "Register `D3PCR2L` writer"]
pub type W = crate::W<D3pcr2lSpec>;
#[doc = "Field `PCS34` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type Pcs34R = crate::FieldReader;
#[doc = "Field `PCS34` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type Pcs34W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS35` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type Pcs35R = crate::FieldReader;
#[doc = "Field `PCS35` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type Pcs35W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS41` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type Pcs41R = crate::FieldReader;
#[doc = "Field `PCS41` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
pub type Pcs41W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&self) -> Pcs34R {
        Pcs34R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&self) -> Pcs35R {
        Pcs35R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&self) -> Pcs41R {
        Pcs41R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&mut self) -> Pcs34W<D3pcr2lSpec> {
        Pcs34W::new(self, 4)
    }
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&mut self) -> Pcs35W<D3pcr2lSpec> {
        Pcs35W::new(self, 6)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&mut self) -> Pcs41W<D3pcr2lSpec> {
        Pcs41W::new(self, 18)
    }
}
#[doc = "EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr2l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pcr2lSpec;
impl crate::RegisterSpec for D3pcr2lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr2l::R`](R) reader structure"]
impl crate::Readable for D3pcr2lSpec {}
#[doc = "`write(|w| ..)` method takes [`d3pcr2l::W`](W) writer structure"]
impl crate::Writable for D3pcr2lSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PCR2L to value 0"]
impl crate::Resettable for D3pcr2lSpec {}
