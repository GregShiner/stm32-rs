#[doc = "Register `D3PMR3` reader"]
pub type R = crate::R<D3pmr3Spec>;
#[doc = "Register `D3PMR3` writer"]
pub type W = crate::W<D3pmr3Spec>;
#[doc = "Field `MR88` reader - D3 Pending Mask on Event input x+64"]
pub type Mr88R = crate::BitReader;
#[doc = "Field `MR88` writer - D3 Pending Mask on Event input x+64"]
pub type Mr88W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> Mr88R {
        Mr88R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - D3 Pending Mask on Event input x+64"]
    #[inline(always)]
    pub fn mr88(&mut self) -> Mr88W<D3pmr3Spec> {
        Mr88W::new(self, 24)
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pmr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pmr3Spec;
impl crate::RegisterSpec for D3pmr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr3::R`](R) reader structure"]
impl crate::Readable for D3pmr3Spec {}
#[doc = "`write(|w| ..)` method takes [`d3pmr3::W`](W) writer structure"]
impl crate::Writable for D3pmr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PMR3 to value 0"]
impl crate::Resettable for D3pmr3Spec {}
