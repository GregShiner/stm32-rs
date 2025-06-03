#[doc = "Register `D3PCR3H` reader"]
pub type R = crate::R<D3pcr3hSpec>;
#[doc = "Register `D3PCR3H` writer"]
pub type W = crate::W<D3pcr3hSpec>;
#[doc = "Field `PCS88` reader - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
pub type Pcs88R = crate::FieldReader;
#[doc = "Field `PCS88` writer - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
pub type Pcs88W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&self) -> Pcs88R {
        Pcs88R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&mut self) -> Pcs88W<D3pcr3hSpec> {
        Pcs88W::new(self, 18)
    }
}
#[doc = "EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr3h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr3h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pcr3hSpec;
impl crate::RegisterSpec for D3pcr3hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pcr3h::R`](R) reader structure"]
impl crate::Readable for D3pcr3hSpec {}
#[doc = "`write(|w| ..)` method takes [`d3pcr3h::W`](W) writer structure"]
impl crate::Writable for D3pcr3hSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PCR3H to value 0"]
impl crate::Resettable for D3pcr3hSpec {}
