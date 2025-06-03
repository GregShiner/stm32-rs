#[doc = "Register `PCR` reader"]
pub type R = crate::R<PcrSpec>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PcrSpec>;
#[doc = "Field `PWAITEN` reader - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
pub type PwaitenR = crate::BitReader;
#[doc = "Field `PWAITEN` writer - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
pub type PwaitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBKEN` reader - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
pub type PbkenR = crate::BitReader;
#[doc = "Field `PBKEN` writer - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
pub type PbkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWID` reader - Data bus width. These bits define the external memory device width."]
pub type PwidR = crate::FieldReader;
#[doc = "Field `PWID` writer - Data bus width. These bits define the external memory device width."]
pub type PwidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCEN` reader - ECC computation logic enable bit"]
pub type EccenR = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECC computation logic enable bit"]
pub type EccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLR` reader - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub type TclrR = crate::FieldReader;
#[doc = "Field `TCLR` writer - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub type TclrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub type TarR = crate::FieldReader;
#[doc = "Field `TAR` writer - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCPS` reader - ECC page size. These bits define the page size for the extended ECC:"]
pub type EccpsR = crate::FieldReader;
#[doc = "Field `ECCPS` writer - ECC page size. These bits define the page size for the extended ECC:"]
pub type EccpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PwaitenR {
        PwaitenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
    #[inline(always)]
    pub fn pbken(&self) -> PbkenR {
        PbkenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data bus width. These bits define the external memory device width."]
    #[inline(always)]
    pub fn pwid(&self) -> PwidR {
        PwidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    pub fn eccen(&self) -> EccenR {
        EccenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tclr(&self) -> TclrR {
        TclrR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:"]
    #[inline(always)]
    pub fn eccps(&self) -> EccpsR {
        EccpsR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable bit. This bit enables the Wait feature for the NAND Flash memory bank:"]
    #[inline(always)]
    pub fn pwaiten(&mut self) -> PwaitenW<PcrSpec> {
        PwaitenW::new(self, 1)
    }
    #[doc = "Bit 2 - NAND Flash memory bank enable bit. This bit enables the memory bank. Accessing a disabled memory bank causes an ERROR on AXI bus"]
    #[inline(always)]
    pub fn pbken(&mut self) -> PbkenW<PcrSpec> {
        PbkenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Data bus width. These bits define the external memory device width."]
    #[inline(always)]
    pub fn pwid(&mut self) -> PwidW<PcrSpec> {
        PwidW::new(self, 4)
    }
    #[doc = "Bit 6 - ECC computation logic enable bit"]
    #[inline(always)]
    pub fn eccen(&mut self) -> EccenW<PcrSpec> {
        EccenW::new(self, 6)
    }
    #[doc = "Bits 9:12 - CLE to RE delay. These bits set time from CLE low to RE low in number of KCK_FMC clock cycles. The time is give by the following formula: t_clr = (TCLR + SET + 2) TKCK_FMC where TKCK_FMC is the KCK_FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tclr(&mut self) -> TclrW<PcrSpec> {
        TclrW::new(self, 9)
    }
    #[doc = "Bits 13:16 - ALE to RE delay. These bits set time from ALE low to RE low in number of KCK_FMC clock cycles. Time is: t_ar = (TAR + SET + 2) TKCK_FMC where TKCK_FMC is the FMC clock period Note: Set is MEMSET or ATTSET according to the addressed space."]
    #[inline(always)]
    pub fn tar(&mut self) -> TarW<PcrSpec> {
        TarW::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECC page size. These bits define the page size for the extended ECC:"]
    #[inline(always)]
    pub fn eccps(&mut self) -> EccpsW<PcrSpec> {
        EccpsW::new(self, 17)
    }
}
#[doc = "NAND Flash control registers\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcrSpec;
impl crate::RegisterSpec for PcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCR to value 0x18"]
impl crate::Resettable for PcrSpec {
    const RESET_VALUE: u32 = 0x18;
}
