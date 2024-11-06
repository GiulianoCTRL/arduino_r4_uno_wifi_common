#[doc = "Register `SSLND` reader"]
pub type R = crate::R<SSLND_SPEC>;
#[doc = "Register `SSLND` writer"]
pub type W = crate::W<SSLND_SPEC>;
#[doc = "Field `SLNDL` reader - SSL Negation Delay Setting"]
pub type SLNDL_R = crate::FieldReader<SLNDL_A>;
#[doc = "SSL Negation Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLNDL_A {
    #[doc = "0: 1 RSPCK"]
    _000 = 0,
    #[doc = "1: 2 RSPCK"]
    _001 = 1,
    #[doc = "2: 3 RSPCK"]
    _010 = 2,
    #[doc = "3: 4 RSPCK"]
    _011 = 3,
    #[doc = "4: 5 RSPCK"]
    _100 = 4,
    #[doc = "5: 6 RSPCK"]
    _101 = 5,
    #[doc = "6: 7 RSPCK"]
    _110 = 6,
    #[doc = "7: 8 RSPCK"]
    _111 = 7,
}
impl From<SLNDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SLNDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SLNDL_A {
    type Ux = u8;
}
impl SLNDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLNDL_A {
        match self.bits {
            0 => SLNDL_A::_000,
            1 => SLNDL_A::_001,
            2 => SLNDL_A::_010,
            3 => SLNDL_A::_011,
            4 => SLNDL_A::_100,
            5 => SLNDL_A::_101,
            6 => SLNDL_A::_110,
            7 => SLNDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SLNDL_A::_000
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SLNDL_A::_001
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SLNDL_A::_010
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SLNDL_A::_011
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SLNDL_A::_100
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SLNDL_A::_101
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SLNDL_A::_110
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SLNDL_A::_111
    }
}
#[doc = "Field `SLNDL` writer - SSL Negation Delay Setting"]
pub type SLNDL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SLNDL_A>;
impl<'a, REG, const O: u8> SLNDL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_000)
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_001)
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_010)
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_011)
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_100)
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_101)
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_110)
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SLNDL_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - SSL Negation Delay Setting"]
    #[inline(always)]
    pub fn slndl(&self) -> SLNDL_R {
        SLNDL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - SSL Negation Delay Setting"]
    #[inline(always)]
    #[must_use]
    pub fn slndl(&mut self) -> SLNDL_W<SSLND_SPEC, 0> {
        SLNDL_W::new(self)
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
#[doc = "SPI Slave Select Negation Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sslnd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sslnd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSLND_SPEC;
impl crate::RegisterSpec for SSLND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sslnd::R`](R) reader structure"]
impl crate::Readable for SSLND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sslnd::W`](W) writer structure"]
impl crate::Writable for SSLND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSLND to value 0"]
impl crate::Resettable for SSLND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
