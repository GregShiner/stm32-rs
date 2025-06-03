#[doc = "Register `ARGR` reader"]
pub type R = crate::R<ArgrSpec>;
#[doc = "Register `ARGR` writer"]
pub type W = crate::W<ArgrSpec>;
#[doc = "Field `CMDARG` reader - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
pub type CmdargR = crate::FieldReader<u32>;
#[doc = "Field `CMDARG` writer - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
pub type CmdargW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CmdargR {
        CmdargR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CmdargW<ArgrSpec> {
        CmdargW::new(self, 0)
    }
}
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nYou can [`read`](crate::Reg::read) this register and get [`argr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArgrSpec;
impl crate::RegisterSpec for ArgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argr::R`](R) reader structure"]
impl crate::Readable for ArgrSpec {}
#[doc = "`write(|w| ..)` method takes [`argr::W`](W) writer structure"]
impl crate::Writable for ArgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARGR to value 0"]
impl crate::Resettable for ArgrSpec {}
