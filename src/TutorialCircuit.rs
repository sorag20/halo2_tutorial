use std::marker::PhantomData;
use halo2_proofs::circuit::Value;
use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Cell, Chip, Layouter, SimpleFloorPlanner},
    plonk::{Advice, Assigned, Circuit, Column, ConstraintSystem, Error, Fixed, Instance},
    poly::Rotation,
};

trait TutorialComposer<F: FieldExt> {
    fn raw_add<FM>()
    fn raw_multiply<FM>()
    fn copy()
    fn expose_public()
    }

    fn raw_add<FM>(
        &self,
        layouter: &mut impl Layouter<F>,
        f: F,
    ) -> Result<(Cell, Cell, Cell), Error>
    where
        FM: FnMut() -> Value<(Assigned<F>, Assigned<F>, Assigned<F>)>;
