#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `CKMODE` reader - indicates the level that clk takes between command"]
pub type CkmodeR = crate::BitReader;
#[doc = "Field `CKMODE` writer - indicates the level that clk takes between command"]
pub type CkmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSHT` reader - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0."]
pub type CshtR = crate::FieldReader;
#[doc = "Field `CSHT` writer - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0."]
pub type CshtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FSIZE` reader - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\\[FSIZE+1\\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0."]
pub type FsizeR = crate::FieldReader;
#[doc = "Field `FSIZE` writer - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\\[FSIZE+1\\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0."]
pub type FsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - indicates the level that clk takes between command"]
    #[inline(always)]
    pub fn ckmode(&self) -> CkmodeR {
        CkmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn csht(&self) -> CshtR {
        CshtR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\\[FSIZE+1\\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn fsize(&self) -> FsizeR {
        FsizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - indicates the level that clk takes between command"]
    #[inline(always)]
    pub fn ckmode(&mut self) -> CkmodeW<DcrSpec> {
        CkmodeW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Chip select high time CSHT+1 defines the minimum number of CLK cycles which the chip select (nCS) must remain high between commands issued to the Flash memory. ... This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn csht(&mut self) -> CshtW<DcrSpec> {
        CshtW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Flash memory size This field defines the size of external memory using the following formula: Number of bytes in Flash memory = 2\\[FSIZE+1\\] FSIZE+1 is effectively the number of address bits required to address the Flash memory. The Flash memory capacity can be up to 4GB (addressed using 32 bits) in indirect mode, but the addressable space in memory-mapped mode is limited to 256MB. If DFM = 1, FSIZE indicates the total capacity of the two Flash memories together. This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn fsize(&mut self) -> FsizeW<DcrSpec> {
        FsizeW::new(self, 16)
    }
}
#[doc = "QUADSPI device configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
