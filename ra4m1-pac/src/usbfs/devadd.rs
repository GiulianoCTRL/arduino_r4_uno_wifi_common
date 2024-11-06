#[doc = "Register `DEVADD%s` reader"]
pub type R = crate::R<DEVADD_SPEC>;
#[doc = "Register `DEVADD%s` writer"]
pub type W = crate::W<DEVADD_SPEC>;
#[doc = "Field `USBSPD` reader - Transfer Speed of Communication Target Device"]
pub type USBSPD_R = crate::FieldReader<USBSPD_A>;
#[doc = "Transfer Speed of Communication Target Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSPD_A {
    #[doc = "0: DEVADDn is not used"]
    _00 = 0,
    #[doc = "1: Low speed"]
    _01 = 1,
    #[doc = "2: Full speed"]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<USBSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBSPD_A {
    type Ux = u8;
}
impl USBSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBSPD_A {
        match self.bits {
            0 => USBSPD_A::_00,
            1 => USBSPD_A::_01,
            2 => USBSPD_A::_10,
            3 => USBSPD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "DEVADDn is not used"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == USBSPD_A::_00
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == USBSPD_A::_01
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == USBSPD_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == USBSPD_A::_11
    }
}
#[doc = "Field `USBSPD` writer - Transfer Speed of Communication Target Device"]
pub type USBSPD_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, USBSPD_A>;
impl<'a, REG, const O: u8> USBSPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DEVADDn is not used"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_00)
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_01)
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(USBSPD_A::_11)
    }
}
impl R {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    pub fn usbspd(&self) -> USBSPD_R {
        USBSPD_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Transfer Speed of Communication Target Device"]
    #[inline(always)]
    #[must_use]
    pub fn usbspd(&mut self) -> USBSPD_W<DEVADD_SPEC, 6> {
        USBSPD_W::new(self)
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
#[doc = "Device Address %s Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devadd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devadd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVADD_SPEC;
impl crate::RegisterSpec for DEVADD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`devadd::R`](R) reader structure"]
impl crate::Readable for DEVADD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`devadd::W`](W) writer structure"]
impl crate::Writable for DEVADD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVADD%s to value 0"]
impl crate::Resettable for DEVADD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
