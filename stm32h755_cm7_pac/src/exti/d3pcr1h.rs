#[doc = "Register `D3PCR1H` reader"]
pub type R = crate::R<D3pcr1hSpec>;
#[doc = "Register `D3PCR1H` writer"]
pub type W = crate::W<D3pcr1hSpec>;
#[doc = "Field `PCS19` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs19R = crate::FieldReader;
#[doc = "Field `PCS19` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs19W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS20` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs20R = crate::FieldReader;
#[doc = "Field `PCS20` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs20W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS21` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs21R = crate::FieldReader;
#[doc = "Field `PCS21` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs21W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS25` reader - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs25R = crate::FieldReader;
#[doc = "Field `PCS25` writer - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
pub type Pcs25W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&self) -> Pcs19R {
        Pcs19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&self) -> Pcs20R {
        Pcs20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&self) -> Pcs21R {
        Pcs21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&self) -> Pcs25R {
        Pcs25R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&mut self) -> Pcs19W<D3pcr1hSpec> {
        Pcs19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&mut self) -> Pcs20W<D3pcr1hSpec> {
        Pcs20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&mut self) -> Pcs21W<D3pcr1hSpec> {
        Pcs21W::new(self, 10)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&mut self) -> Pcs25W<D3pcr1hSpec> {
        Pcs25W::new(self, 18)
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr1h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pcr1hSpec;
impl crate::RegisterSpec for D3pcr1hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr1h::R`](R) reader structure"]
impl crate::Readable for D3pcr1hSpec {}
#[doc = "`write(|w| ..)` method takes [`d3pcr1h::W`](W) writer structure"]
impl crate::Writable for D3pcr1hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PCR1H to value 0"]
impl crate::Resettable for D3pcr1hSpec {}
