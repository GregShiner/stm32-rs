#[doc = "Register `D3PMR2` reader"]
pub type R = crate::R<D3pmr2Spec>;
#[doc = "Register `D3PMR2` writer"]
pub type W = crate::W<D3pmr2Spec>;
#[doc = "Field `MR34` reader - D3 Pending Mask on Event input x+32"]
pub type Mr34R = crate::BitReader;
#[doc = "Field `MR34` writer - D3 Pending Mask on Event input x+32"]
pub type Mr34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR35` reader - D3 Pending Mask on Event input x+32"]
pub type Mr35R = crate::BitReader;
#[doc = "Field `MR35` writer - D3 Pending Mask on Event input x+32"]
pub type Mr35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR41` reader - D3 Pending Mask on Event input x+32"]
pub type Mr41R = crate::BitReader;
#[doc = "Field `MR41` writer - D3 Pending Mask on Event input x+32"]
pub type Mr41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR48` reader - D3 Pending Mask on Event input x+32"]
pub type Mr48R = crate::BitReader;
#[doc = "Field `MR48` writer - D3 Pending Mask on Event input x+32"]
pub type Mr48W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR49` reader - D3 Pending Mask on Event input x+32"]
pub type Mr49R = crate::BitReader;
#[doc = "Field `MR49` writer - D3 Pending Mask on Event input x+32"]
pub type Mr49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR50` reader - D3 Pending Mask on Event input x+32"]
pub type Mr50R = crate::BitReader;
#[doc = "Field `MR50` writer - D3 Pending Mask on Event input x+32"]
pub type Mr50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR51` reader - D3 Pending Mask on Event input x+32"]
pub type Mr51R = crate::BitReader;
#[doc = "Field `MR51` writer - D3 Pending Mask on Event input x+32"]
pub type Mr51W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR52` reader - D3 Pending Mask on Event input x+32"]
pub type Mr52R = crate::BitReader;
#[doc = "Field `MR52` writer - D3 Pending Mask on Event input x+32"]
pub type Mr52W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR53` reader - D3 Pending Mask on Event input x+32"]
pub type Mr53R = crate::BitReader;
#[doc = "Field `MR53` writer - D3 Pending Mask on Event input x+32"]
pub type Mr53W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr34(&self) -> Mr34R {
        Mr34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr35(&self) -> Mr35R {
        Mr35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr41(&self) -> Mr41R {
        Mr41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr48(&self) -> Mr48R {
        Mr48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr49(&self) -> Mr49R {
        Mr49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr50(&self) -> Mr50R {
        Mr50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr51(&self) -> Mr51R {
        Mr51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr52(&self) -> Mr52R {
        Mr52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr53(&self) -> Mr53R {
        Mr53R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr34(&mut self) -> Mr34W<D3pmr2Spec> {
        Mr34W::new(self, 2)
    }
    #[doc = "Bit 3 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr35(&mut self) -> Mr35W<D3pmr2Spec> {
        Mr35W::new(self, 3)
    }
    #[doc = "Bit 9 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr41(&mut self) -> Mr41W<D3pmr2Spec> {
        Mr41W::new(self, 9)
    }
    #[doc = "Bit 16 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr48(&mut self) -> Mr48W<D3pmr2Spec> {
        Mr48W::new(self, 16)
    }
    #[doc = "Bit 17 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr49(&mut self) -> Mr49W<D3pmr2Spec> {
        Mr49W::new(self, 17)
    }
    #[doc = "Bit 18 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr50(&mut self) -> Mr50W<D3pmr2Spec> {
        Mr50W::new(self, 18)
    }
    #[doc = "Bit 19 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr51(&mut self) -> Mr51W<D3pmr2Spec> {
        Mr51W::new(self, 19)
    }
    #[doc = "Bit 20 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr52(&mut self) -> Mr52W<D3pmr2Spec> {
        Mr52W::new(self, 20)
    }
    #[doc = "Bit 21 - D3 Pending Mask on Event input x+32"]
    #[inline(always)]
    pub fn mr53(&mut self) -> Mr53W<D3pmr2Spec> {
        Mr53W::new(self, 21)
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pmr2Spec;
impl crate::RegisterSpec for D3pmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr2::R`](R) reader structure"]
impl crate::Readable for D3pmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`d3pmr2::W`](W) writer structure"]
impl crate::Writable for D3pmr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PMR2 to value 0"]
impl crate::Resettable for D3pmr2Spec {}
