#[doc = "Register `OPTSR_CUR_` reader"]
pub type R = crate::R<OptsrCur_Spec>;
#[doc = "Register `OPTSR_CUR_` writer"]
pub type W = crate::W<OptsrCur_Spec>;
#[doc = "Field `OPT_BUSY` reader - Option byte change ongoing flag"]
pub type OptBusyR = crate::BitReader;
#[doc = "Field `OPT_BUSY` writer - Option byte change ongoing flag"]
pub type OptBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR_LEV` reader - Brownout level option status bit"]
pub type BorLevR = crate::FieldReader;
#[doc = "Field `BOR_LEV` writer - Brownout level option status bit"]
pub type BorLevW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IWDG1_HW` reader - IWDG1 control option status bit"]
pub type Iwdg1HwR = crate::BitReader;
#[doc = "Field `IWDG1_HW` writer - IWDG1 control option status bit"]
pub type Iwdg1HwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STOP_D1` reader - D1 DStop entry reset option status bit"]
pub type NRstStopD1R = crate::BitReader;
#[doc = "Field `nRST_STOP_D1` writer - D1 DStop entry reset option status bit"]
pub type NRstStopD1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STBY_D1` reader - D1 DStandby entry reset option status bit"]
pub type NRstStbyD1R = crate::BitReader;
#[doc = "Field `nRST_STBY_D1` writer - D1 DStandby entry reset option status bit"]
pub type NRstStbyD1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Readout protection level option status byte"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Readout protection level option status byte"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FZ_IWDG_STOP` reader - IWDG Stop mode freeze option status bit"]
pub type FzIwdgStopR = crate::BitReader;
#[doc = "Field `FZ_IWDG_STOP` writer - IWDG Stop mode freeze option status bit"]
pub type FzIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FZ_IWDG_SDBY` reader - IWDG Standby mode freeze option status bit"]
pub type FzIwdgSdbyR = crate::BitReader;
#[doc = "Field `FZ_IWDG_SDBY` writer - IWDG Standby mode freeze option status bit"]
pub type FzIwdgSdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST_RAM_SIZE` reader - DTCM RAM size option status"]
pub type StRamSizeR = crate::FieldReader;
#[doc = "Field `ST_RAM_SIZE` writer - DTCM RAM size option status"]
pub type StRamSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECURITY` reader - Security enable option status bit"]
pub type SecurityR = crate::BitReader;
#[doc = "Field `SECURITY` writer - Security enable option status bit"]
pub type SecurityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSS1` reader - User option bit 1"]
pub type Rss1R = crate::BitReader;
#[doc = "Field `RSS1` writer - User option bit 1"]
pub type Rss1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERSO_OK` reader - Device personalization status bit"]
pub type PersoOkR = crate::BitReader;
#[doc = "Field `PERSO_OK` writer - Device personalization status bit"]
pub type PersoOkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IO_HSLV` reader - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
pub type IoHslvR = crate::BitReader;
#[doc = "Field `IO_HSLV` writer - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
pub type IoHslvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPTCHANGEERR` reader - Option byte change error flag"]
pub type OptchangeerrR = crate::BitReader;
#[doc = "Field `OPTCHANGEERR` writer - Option byte change error flag"]
pub type OptchangeerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWAP_BANK_OPT` reader - Bank swapping option status bit"]
pub type SwapBankOptR = crate::BitReader;
#[doc = "Field `SWAP_BANK_OPT` writer - Bank swapping option status bit"]
pub type SwapBankOptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&self) -> OptBusyR {
        OptBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&self) -> BorLevR {
        BorLevR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - IWDG1 control option status bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&self) -> Iwdg1HwR {
        Iwdg1HwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - D1 DStop entry reset option status bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&self) -> NRstStopD1R {
        NRstStopD1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D1 DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&self) -> NRstStbyD1R {
        NRstStbyD1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&self) -> FzIwdgStopR {
        FzIwdgStopR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&self) -> FzIwdgSdbyR {
        FzIwdgSdbyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - DTCM RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&self) -> StRamSizeR {
        StRamSizeR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&self) -> SecurityR {
        SecurityR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - User option bit 1"]
    #[inline(always)]
    pub fn rss1(&self) -> Rss1R {
        Rss1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Device personalization status bit"]
    #[inline(always)]
    pub fn perso_ok(&self) -> PersoOkR {
        PersoOkR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IoHslvR {
        IoHslvR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&self) -> OptchangeerrR {
        OptchangeerrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&self) -> SwapBankOptR {
        SwapBankOptR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Option byte change ongoing flag"]
    #[inline(always)]
    pub fn opt_busy(&mut self) -> OptBusyW<OptsrCur_Spec> {
        OptBusyW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Brownout level option status bit"]
    #[inline(always)]
    pub fn bor_lev(&mut self) -> BorLevW<OptsrCur_Spec> {
        BorLevW::new(self, 2)
    }
    #[doc = "Bit 4 - IWDG1 control option status bit"]
    #[inline(always)]
    pub fn iwdg1_hw(&mut self) -> Iwdg1HwW<OptsrCur_Spec> {
        Iwdg1HwW::new(self, 4)
    }
    #[doc = "Bit 6 - D1 DStop entry reset option status bit"]
    #[inline(always)]
    pub fn n_rst_stop_d1(&mut self) -> NRstStopD1W<OptsrCur_Spec> {
        NRstStopD1W::new(self, 6)
    }
    #[doc = "Bit 7 - D1 DStandby entry reset option status bit"]
    #[inline(always)]
    pub fn n_rst_stby_d1(&mut self) -> NRstStbyD1W<OptsrCur_Spec> {
        NRstStbyD1W::new(self, 7)
    }
    #[doc = "Bits 8:15 - Readout protection level option status byte"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RdpW<OptsrCur_Spec> {
        RdpW::new(self, 8)
    }
    #[doc = "Bit 17 - IWDG Stop mode freeze option status bit"]
    #[inline(always)]
    pub fn fz_iwdg_stop(&mut self) -> FzIwdgStopW<OptsrCur_Spec> {
        FzIwdgStopW::new(self, 17)
    }
    #[doc = "Bit 18 - IWDG Standby mode freeze option status bit"]
    #[inline(always)]
    pub fn fz_iwdg_sdby(&mut self) -> FzIwdgSdbyW<OptsrCur_Spec> {
        FzIwdgSdbyW::new(self, 18)
    }
    #[doc = "Bits 19:20 - DTCM RAM size option status"]
    #[inline(always)]
    pub fn st_ram_size(&mut self) -> StRamSizeW<OptsrCur_Spec> {
        StRamSizeW::new(self, 19)
    }
    #[doc = "Bit 21 - Security enable option status bit"]
    #[inline(always)]
    pub fn security(&mut self) -> SecurityW<OptsrCur_Spec> {
        SecurityW::new(self, 21)
    }
    #[doc = "Bit 26 - User option bit 1"]
    #[inline(always)]
    pub fn rss1(&mut self) -> Rss1W<OptsrCur_Spec> {
        Rss1W::new(self, 26)
    }
    #[doc = "Bit 28 - Device personalization status bit"]
    #[inline(always)]
    pub fn perso_ok(&mut self) -> PersoOkW<OptsrCur_Spec> {
        PersoOkW::new(self, 28)
    }
    #[doc = "Bit 29 - I/O high-speed at low-voltage status bit (PRODUCT_BELOW_25V)"]
    #[inline(always)]
    pub fn io_hslv(&mut self) -> IoHslvW<OptsrCur_Spec> {
        IoHslvW::new(self, 29)
    }
    #[doc = "Bit 30 - Option byte change error flag"]
    #[inline(always)]
    pub fn optchangeerr(&mut self) -> OptchangeerrW<OptsrCur_Spec> {
        OptchangeerrW::new(self, 30)
    }
    #[doc = "Bit 31 - Bank swapping option status bit"]
    #[inline(always)]
    pub fn swap_bank_opt(&mut self) -> SwapBankOptW<OptsrCur_Spec> {
        SwapBankOptW::new(self, 31)
    }
}
#[doc = "FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_cur_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OptsrCur_Spec;
impl crate::RegisterSpec for OptsrCur_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optsr_cur_::R`](R) reader structure"]
impl crate::Readable for OptsrCur_Spec {}
#[doc = "`write(|w| ..)` method takes [`optsr_cur_::W`](W) writer structure"]
impl crate::Writable for OptsrCur_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTSR_CUR_ to value 0"]
impl crate::Resettable for OptsrCur_Spec {}
