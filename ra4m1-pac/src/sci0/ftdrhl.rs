#[doc = "Register `FTDRHL` writer"]
pub type W = crate::W<FTDRHL_SPEC>;
#[doc = "Field `TDAT` writer - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
pub type TDAT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
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
    #[doc = "Bits 0:8 - Serial transmit data (Valid only in asynchronous mode(including multi-processor) or clock synchronous mode, and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn tdat(&mut self) -> TDAT_W<FTDRHL_SPEC, 0> {
        TDAT_W::new(self)
    }
    #[doc = "Bit 9 - Multi-processor transfer bit flag (Valid only in asynchronous mode and SMR.MP=1 and FIFO selected)"]
    #[inline(always)]
    #[must_use]
    pub fn mpbt(&mut self) -> MPBT_W<FTDRHL_SPEC, 9> {
        MPBT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit FIFO Data Register HL\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftdrhl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTDRHL_SPEC;
impl crate::RegisterSpec for FTDRHL_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`ftdrhl::W`](W) writer structure"]
impl crate::Writable for FTDRHL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTDRHL to value 0xffff"]
impl crate::Resettable for FTDRHL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
