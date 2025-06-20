#[doc = "Register `OTG_HS_DCFG` reader"]
pub type R = crate::R<OtgHsDcfgSpec>;
#[doc = "Register `OTG_HS_DCFG` writer"]
pub type W = crate::W<OtgHsDcfgSpec>;
#[doc = "Field `DSPD` reader - Device speed"]
pub type DspdR = crate::FieldReader;
#[doc = "Field `DSPD` writer - Device speed"]
pub type DspdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZLSOHSK` reader - Nonzero-length status OUT handshake"]
pub type NzlsohskR = crate::BitReader;
#[doc = "Field `NZLSOHSK` writer - Nonzero-length status OUT handshake"]
pub type NzlsohskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAD` reader - Device address"]
pub type DadR = crate::FieldReader;
#[doc = "Field `DAD` writer - Device address"]
pub type DadW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PFIVL` reader - Periodic (micro)frame interval"]
pub type PfivlR = crate::FieldReader;
#[doc = "Field `PFIVL` writer - Periodic (micro)frame interval"]
pub type PfivlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PERSCHIVL` reader - Periodic scheduling interval"]
pub type PerschivlR = crate::FieldReader;
#[doc = "Field `PERSCHIVL` writer - Periodic scheduling interval"]
pub type PerschivlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&self) -> DspdR {
        DspdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NzlsohskR {
        NzlsohskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DadR {
        DadR::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&self) -> PfivlR {
        PfivlR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    pub fn perschivl(&self) -> PerschivlR {
        PerschivlR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DspdW<OtgHsDcfgSpec> {
        DspdW::new(self, 0)
    }
    #[doc = "Bit 2 - Nonzero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NzlsohskW<OtgHsDcfgSpec> {
        NzlsohskW::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DadW<OtgHsDcfgSpec> {
        DadW::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic (micro)frame interval"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PfivlW<OtgHsDcfgSpec> {
        PfivlW::new(self, 11)
    }
    #[doc = "Bits 24:25 - Periodic scheduling interval"]
    #[inline(always)]
    pub fn perschivl(&mut self) -> PerschivlW<OtgHsDcfgSpec> {
        PerschivlW::new(self, 24)
    }
}
#[doc = "OTG_HS device configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDcfgSpec;
impl crate::RegisterSpec for OtgHsDcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dcfg::R`](R) reader structure"]
impl crate::Readable for OtgHsDcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dcfg::W`](W) writer structure"]
impl crate::Writable for OtgHsDcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DCFG to value 0x0220_0000"]
impl crate::Resettable for OtgHsDcfgSpec {
    const RESET_VALUE: u32 = 0x0220_0000;
}
