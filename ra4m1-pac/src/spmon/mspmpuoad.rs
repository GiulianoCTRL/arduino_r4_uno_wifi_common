#[doc = "Register `MSPMPUOAD` reader"]
pub type R = crate::R<MSPMPUOAD_SPEC>;
#[doc = "Register `MSPMPUOAD` writer"]
pub type W = crate::W<MSPMPUOAD_SPEC>;
#[doc = "Field `OAD` reader - Operation after detection"]
pub type OAD_R = crate::BitReader<OAD_A>;
#[doc = "Operation after detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OAD_A {
    #[doc = "0: Non-maskable interrupt"]
    _0 = 0,
    #[doc = "1: Reset."]
    _1 = 1,
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
            false => OAD_A::_0,
            true => OAD_A::_1,
        }
    }
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OAD_A::_0
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OAD_A::_1
    }
}
#[doc = "Field `OAD` writer - Operation after detection"]
pub type OAD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OAD_A>;
impl<'a, REG, const O: u8> OAD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-maskable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OAD_A::_1)
    }
}
#[doc = "Write Keyword The data written to these bits are not stored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    #[doc = "165: Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
    _0X_A5 = 165,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for KEY_AW {
    type Ux = u8;
}
#[doc = "Field `KEY` writer - Write Keyword The data written to these bits are not stored."]
pub type KEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, KEY_AW>;
impl<'a, REG, const O: u8> KEY_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing to the OAD bit is valid, when the KEY bits are written 0xA5."]
    #[inline(always)]
    pub fn _0x_a5(self) -> &'a mut crate::W<REG> {
        self.variant(KEY_AW::_0X_A5)
    }
}
impl R {
    #[doc = "Bit 0 - Operation after detection"]
    #[inline(always)]
    pub fn oad(&self) -> OAD_R {
        OAD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operation after detection"]
    #[inline(always)]
    #[must_use]
    pub fn oad(&mut self) -> OAD_W<MSPMPUOAD_SPEC, 0> {
        OAD_W::new(self)
    }
    #[doc = "Bits 8:15 - Write Keyword The data written to these bits are not stored."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<MSPMPUOAD_SPEC, 8> {
        KEY_W::new(self)
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
#[doc = "Stack Pointer Monitor Operation After Detection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mspmpuoad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspmpuoad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSPMPUOAD_SPEC;
impl crate::RegisterSpec for MSPMPUOAD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`mspmpuoad::R`](R) reader structure"]
impl crate::Readable for MSPMPUOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mspmpuoad::W`](W) writer structure"]
impl crate::Writable for MSPMPUOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPMPUOAD to value 0"]
impl crate::Resettable for MSPMPUOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
