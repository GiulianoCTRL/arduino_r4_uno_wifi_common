#[doc = "Register `FTDRH` writer"]
pub type W = crate::W<FTDRH_SPEC>;
#[doc = "Field `TDATH` writer - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TDATH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPBT_AW {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<MPBT_AW> for bool {
    #[inline(always)]
    fn from(variant: MPBT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPBT` writer - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
pub type MPBT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, MPBT_AW>;
impl<'a, REG, const O: u8> MPBT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transmission cycles"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(MPBT_AW::_0)
    }
    #[doc = "ID transmission cycles"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(MPBT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Serial transmit data (b8) (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn tdath(&mut self) -> TDATH_W<FTDRH_SPEC, 0> {
        TDATH_W::new(self)
    }
    #[doc = "Bit 1 - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<FTDRH_SPEC, 1> {
        MPBT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit FIFO Data Register H\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftdrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTDRH_SPEC;
impl crate::RegisterSpec for FTDRH_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`ftdrh::W`](W) writer structure"]
impl crate::Writable for FTDRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTDRH to value 0xff"]
impl crate::Resettable for FTDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
