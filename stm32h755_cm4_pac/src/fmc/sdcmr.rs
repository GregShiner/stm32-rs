#[doc = "Register `SDCMR` reader"]
pub type R = crate::R<SdcmrSpec>;
#[doc = "Register `SDCMR` writer"]
pub type W = crate::W<SdcmrSpec>;
#[doc = "Field `MODE` reader - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTB2` reader - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub type Ctb2R = crate::BitReader;
#[doc = "Field `CTB2` writer - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
pub type Ctb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTB1` reader - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub type Ctb1R = crate::BitReader;
#[doc = "Field `CTB1` writer - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
pub type Ctb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NRFS` reader - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
pub type NrfsR = crate::FieldReader;
#[doc = "Field `NRFS` writer - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
pub type NrfsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MRD` reader - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\] bits are also used to program the extended mode register for mobile SDRAM."]
pub type MrdR = crate::FieldReader<u16>;
#[doc = "Field `MRD` writer - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\] bits are also used to program the extended mode register for mobile SDRAM."]
pub type MrdW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    pub fn ctb2(&self) -> Ctb2R {
        Ctb2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    pub fn ctb1(&self) -> Ctb1R {
        Ctb1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
    #[inline(always)]
    pub fn nrfs(&self) -> NrfsR {
        NrfsR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\] bits are also used to program the extended mode register for mobile SDRAM."]
    #[inline(always)]
    pub fn mrd(&self) -> MrdR {
        MrdR::new(((self.bits >> 9) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Command mode These bits define the command issued to the SDRAM device. Note: When a command is issued, at least one Command Target Bank bit ( CTB1 or CTB2) must be set otherwise the command will be ignored. Note: If two SDRAM banks are used, the Auto-refresh and PALL command must be issued simultaneously to the two devices with CTB1 and CTB2 bits set otherwise the command will be ignored. Note: If only one SDRAM bank is used and a command is issued with its associated CTB bit set, the other CTB bit of the unused bank must be kept to 0."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<SdcmrSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Command Target Bank 2 This bit indicates whether the command will be issued to SDRAM Bank 2 or not."]
    #[inline(always)]
    pub fn ctb2(&mut self) -> Ctb2W<SdcmrSpec> {
        Ctb2W::new(self, 3)
    }
    #[doc = "Bit 4 - Command Target Bank 1 This bit indicates whether the command will be issued to SDRAM Bank 1 or not."]
    #[inline(always)]
    pub fn ctb1(&mut self) -> Ctb1W<SdcmrSpec> {
        Ctb1W::new(self, 4)
    }
    #[doc = "Bits 5:8 - Number of Auto-refresh These bits define the number of consecutive Auto-refresh commands issued when MODE = 011. ...."]
    #[inline(always)]
    pub fn nrfs(&mut self) -> NrfsW<SdcmrSpec> {
        NrfsW::new(self, 5)
    }
    #[doc = "Bits 9:22 - Mode Register definition This 14-bit field defines the SDRAM Mode Register content. The Mode Register is programmed using the Load Mode Register command. The MRD\\[13:0\\] bits are also used to program the extended mode register for mobile SDRAM."]
    #[inline(always)]
    pub fn mrd(&mut self) -> MrdW<SdcmrSpec> {
        MrdW::new(self, 9)
    }
}
#[doc = "This register contains the command issued when the SDRAM device is accessed. This register is used to initialize the SDRAM device, and to activate the Self-refresh and the Power-down modes. As soon as the MODE field is written, the command will be issued only to one or to both SDRAM banks according to CTB1 and CTB2 command bits. This register is the same for both SDRAM banks.\n\nYou can [`read`](crate::Reg::read) this register and get [`sdcmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdcmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdcmrSpec;
impl crate::RegisterSpec for SdcmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdcmr::R`](R) reader structure"]
impl crate::Readable for SdcmrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdcmr::W`](W) writer structure"]
impl crate::Writable for SdcmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDCMR to value 0"]
impl crate::Resettable for SdcmrSpec {}
