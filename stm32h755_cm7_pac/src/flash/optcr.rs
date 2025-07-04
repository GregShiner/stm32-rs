#[doc = "Register `OPTCR` reader"]
pub type R = crate::R<OptcrSpec>;
#[doc = "Register `OPTCR` writer"]
pub type W = crate::W<OptcrSpec>;
#[doc = "Field `OPTLOCK` reader - FLASH_OPTCR lock option configuration bit"]
pub type OptlockR = crate::BitReader;
#[doc = "Field `OPTLOCK` writer - FLASH_OPTCR lock option configuration bit"]
pub type OptlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTSTART` reader - Option byte start change option configuration bit"]
pub type OptstartR = crate::BitReader;
#[doc = "Field `OPTSTART` writer - Option byte start change option configuration bit"]
pub type OptstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MER` reader - Flash mass erase enable bit"]
pub type MerR = crate::BitReader;
#[doc = "Field `MER` writer - Flash mass erase enable bit"]
pub type MerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTCHANGEERRIE` reader - Option byte change error interrupt enable bit"]
pub type OptchangeerrieR = crate::BitReader;
#[doc = "Field `OPTCHANGEERRIE` writer - Option byte change error interrupt enable bit"]
pub type OptchangeerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK` reader - Bank swapping configuration bit"]
pub type SwapBankR = crate::BitReader;
#[doc = "Field `SWAP_BANK` writer - Bank swapping configuration bit"]
pub type SwapBankW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    pub fn optlock(&self) -> OptlockR {
        OptlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&self) -> OptstartR {
        OptstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    pub fn mer(&self) -> MerR {
        MerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&self) -> OptchangeerrieR {
        OptchangeerrieR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&self) -> SwapBankR {
        SwapBankR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH_OPTCR lock option configuration bit"]
    #[inline(always)]
    pub fn optlock(&mut self) -> OptlockW<OptcrSpec> {
        OptlockW::new(self, 0)
    }
    #[doc = "Bit 1 - Option byte start change option configuration bit"]
    #[inline(always)]
    pub fn optstart(&mut self) -> OptstartW<OptcrSpec> {
        OptstartW::new(self, 1)
    }
    #[doc = "Bit 4 - Flash mass erase enable bit"]
    #[inline(always)]
    pub fn mer(&mut self) -> MerW<OptcrSpec> {
        MerW::new(self, 4)
    }
    #[doc = "Bit 30 - Option byte change error interrupt enable bit"]
    #[inline(always)]
    pub fn optchangeerrie(&mut self) -> OptchangeerrieW<OptcrSpec> {
        OptchangeerrieW::new(self, 30)
    }
    #[doc = "Bit 31 - Bank swapping configuration bit"]
    #[inline(always)]
    pub fn swap_bank(&mut self) -> SwapBankW<OptcrSpec> {
        SwapBankW::new(self, 31)
    }
}
#[doc = "FLASH option control register\n\nYou can [`read`](crate::Reg::read) this register and get [`optcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptcrSpec;
impl crate::RegisterSpec for OptcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr::R`](R) reader structure"]
impl crate::Readable for OptcrSpec {}
#[doc = "`write(|w| ..)` method takes [`optcr::W`](W) writer structure"]
impl crate::Writable for OptcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTCR to value 0"]
impl crate::Resettable for OptcrSpec {}
