#[doc = "Register `CPUIMR3` reader"]
pub type R = crate::R<Cpuimr3Spec>;
#[doc = "Field `MR64` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr64R = crate::BitReader;
#[doc = "Field `MR65` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr65R = crate::BitReader;
#[doc = "Field `MR66` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr66R = crate::BitReader;
#[doc = "Field `MR67` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr67R = crate::BitReader;
#[doc = "Field `MR68` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr68R = crate::BitReader;
#[doc = "Field `MR69` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr69R = crate::BitReader;
#[doc = "Field `MR70` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr70R = crate::BitReader;
#[doc = "Field `MR71` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr71R = crate::BitReader;
#[doc = "Field `MR72` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr72R = crate::BitReader;
#[doc = "Field `MR73` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr73R = crate::BitReader;
#[doc = "Field `MR74` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr74R = crate::BitReader;
#[doc = "Field `MR75` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr75R = crate::BitReader;
#[doc = "Field `MR76` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr76R = crate::BitReader;
#[doc = "Field `MR77` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr77R = crate::BitReader;
#[doc = "Field `MR78` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr78R = crate::BitReader;
#[doc = "Field `MR79` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr79R = crate::BitReader;
#[doc = "Field `MR80` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr80R = crate::BitReader;
#[doc = "Field `MR82` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr82R = crate::BitReader;
#[doc = "Field `MR84` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr84R = crate::BitReader;
#[doc = "Field `MR85` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr85R = crate::BitReader;
#[doc = "Field `MR86` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr86R = crate::BitReader;
#[doc = "Field `MR87` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr87R = crate::BitReader;
#[doc = "Field `MR88` reader - CPU Interrupt Mask on Direct Event input x+64"]
pub type Mr88R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr64(&self) -> Mr64R {
        Mr64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr65(&self) -> Mr65R {
        Mr65R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr66(&self) -> Mr66R {
        Mr66R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr67(&self) -> Mr67R {
        Mr67R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr68(&self) -> Mr68R {
        Mr68R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr69(&self) -> Mr69R {
        Mr69R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr70(&self) -> Mr70R {
        Mr70R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr71(&self) -> Mr71R {
        Mr71R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr72(&self) -> Mr72R {
        Mr72R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr73(&self) -> Mr73R {
        Mr73R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr74(&self) -> Mr74R {
        Mr74R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr75(&self) -> Mr75R {
        Mr75R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr76(&self) -> Mr76R {
        Mr76R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr77(&self) -> Mr77R {
        Mr77R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr78(&self) -> Mr78R {
        Mr78R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr79(&self) -> Mr79R {
        Mr79R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr80(&self) -> Mr80R {
        Mr80R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr82(&self) -> Mr82R {
        Mr82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr84(&self) -> Mr84R {
        Mr84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr85(&self) -> Mr85R {
        Mr85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr86(&self) -> Mr86R {
        Mr86R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr87(&self) -> Mr87R {
        Mr87R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CPU Interrupt Mask on Direct Event input x+64"]
    #[inline(always)]
    pub fn mr88(&self) -> Mr88R {
        Mr88R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "EXTI interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuimr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuimr3Spec;
impl crate::RegisterSpec for Cpuimr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuimr3::R`](R) reader structure"]
impl crate::Readable for Cpuimr3Spec {}
#[doc = "`reset()` method sets CPUIMR3 to value 0"]
impl crate::Resettable for Cpuimr3Spec {}
