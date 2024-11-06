#[doc = "Register `PARIOAD` reader"]
pub type R = crate::R<PARIOAD_SPEC>;
#[doc = "Register `PARIOAD` writer"]
pub type W = crate::W<PARIOAD_SPEC>;
#[doc = "Field `OAD` reader - Operation after Detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "1: Reset"]
    _1 = 1,
    #[doc = "0: Non maskable interrupt."]
    _0 = 0,
}
impl From<OAD_A> for bool {
    #[inline(always)]
    fn from(variant: OAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OAD_A {
        match self.bits {
            true => OAD_A::_1,
            false => OAD_A::_0,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
    #[doc = "Non maskable interrupt."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
}
#[doc = "Field `OAD` writer - Operation after Detection"]
pub type OAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OAD_A>;
impl<'a, REG, const O: u8> OAD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_1)
    }
    #[doc = "Non maskable interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_0)
    }
}
impl R {
    #[doc = "Bit 0 - Operation after Detection"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation after Detection"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<PARIOAD_SPEC, 0> {
        OAD_W::new(self)
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
#[doc = "SRAM Parity Error Operation After Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`parioad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parioad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARIOAD_SPEC;
impl crate::RegisterSpec for PARIOAD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`parioad::R`](R) reader structure"]
impl crate::Readable for PARIOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`parioad::W`](W) writer structure"]
impl crate::Writable for PARIOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARIOAD to value 0"]
impl crate::Resettable for PARIOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
