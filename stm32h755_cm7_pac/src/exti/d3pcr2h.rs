#[doc = "Register `D3PCR2H` reader"]
pub type R = crate::R<D3pcr2hSpec>;
#[doc = "Register `D3PCR2H` writer"]
pub type W = crate::W<D3pcr2hSpec>;
#[doc = "Field `PCS48` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs48R = crate::FieldReader;
#[doc = "Field `PCS48` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs48W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS49` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs49R = crate::FieldReader;
#[doc = "Field `PCS49` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs49W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS50` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs50R = crate::FieldReader;
#[doc = "Field `PCS50` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs50W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS51` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs51R = crate::FieldReader;
#[doc = "Field `PCS51` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs51W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS52` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs52R = crate::FieldReader;
#[doc = "Field `PCS52` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs52W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCS53` reader - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs53R = crate::FieldReader;
#[doc = "Field `PCS53` writer - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
pub type Pcs53W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&self) -> Pcs48R {
        Pcs48R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&self) -> Pcs49R {
        Pcs49R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&self) -> Pcs50R {
        Pcs50R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&self) -> Pcs51R {
        Pcs51R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&self) -> Pcs52R {
        Pcs52R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&self) -> Pcs53R {
        Pcs53R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&mut self) -> Pcs48W<D3pcr2hSpec> {
        Pcs48W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&mut self) -> Pcs49W<D3pcr2hSpec> {
        Pcs49W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&mut self) -> Pcs50W<D3pcr2hSpec> {
        Pcs50W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&mut self) -> Pcs51W<D3pcr2hSpec> {
        Pcs51W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&mut self) -> Pcs52W<D3pcr2hSpec> {
        Pcs52W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&mut self) -> Pcs53W<D3pcr2hSpec> {
        Pcs53W::new(self, 10)
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr2h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pcr2hSpec;
impl crate::RegisterSpec for D3pcr2hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr2h::R`](R) reader structure"]
impl crate::Readable for D3pcr2hSpec {}
#[doc = "`write(|w| ..)` method takes [`d3pcr2h::W`](W) writer structure"]
impl crate::Writable for D3pcr2hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PCR2H to value 0"]
impl crate::Resettable for D3pcr2hSpec {}
