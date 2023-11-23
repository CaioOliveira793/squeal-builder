use alloc::string::String;

use super::{Columns, FromTable, Tables};
use crate::{error::SqlError, macros::map_intermediate_sql};

pub struct SelectColumn<Arg> {
    pub(super) command: String,
    pub(super) arguments: Arg,
}

impl<Arg> SelectColumn<Arg> {
    /// Add a column into the SELECT command
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use squeal_builder::select::*;
    /// let cmd = select(arguments)
    ///     .column("first_name")?
    ///     .column("last_name")?;
    ///
    /// assert_eq!(cmd.as_str(), "SELECT first_name, last_name");
    /// ```
    pub fn column<EArg>(mut self, column: &str) -> Result<Self, SqlError<EArg>> {
        self.command.push(',');
        self.transition_column(column)
    }

    pub(super) fn transition_column<EArg>(mut self, column: &str) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(column.len() + 1)?;
        self.command.push(' ');
        self.command.push_str(column);
        Ok(self)
    }

    /// Add a column with a alias into the SELECT command
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use squeal_builder::select::*;
    /// let cmd = select(arguments)
    ///     .column_as("firstName", "first_name")?
    ///     .column_as("lastName", "last_name")?;
    ///
    /// assert_eq!(cmd.as_str(), "SELECT firstName AS first_name, lastName AS last_name");
    /// ```
    pub fn column_as<EArg>(mut self, column: &str, alias: &str) -> Result<Self, SqlError<EArg>> {
        self.command.push(',');
        self.transition_column_as(column, alias)
    }

    pub(super) fn transition_column_as<EArg>(
        mut self,
        column: &str,
        alias: &str,
    ) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(column.len() + alias.len() + 5)?;
        self.command.push(' ');
        self.command.push_str(column);
        self.command.push_str(" AS ");
        self.command.push_str(alias);
        Ok(self)
    }

    pub fn static_columns<EArg>(mut self, columns: Columns) -> Result<Self, SqlError<EArg>> {
        self.command.push(',');
        self.transition_static_columns(columns)
    }

    pub(super) fn transition_static_columns<EArg>(
        mut self,
        columns: Columns,
    ) -> Result<Self, SqlError<EArg>> {
        self.command.try_reserve(columns.0.len() + 1)?;
        self.command.push(' ');
        self.command.push_str(columns.0);
        Ok(self)
    }

    pub fn from<EArg>(self, table: &str) -> Result<FromTable<Arg>, SqlError<EArg>> {
        let mut sql = map_intermediate_sql!(FromTable, self);
        sql.command.push_str(" FROM");
        sql.transition_from(table)
    }

    pub fn static_from<EArg>(self, tables: Tables) -> Result<FromTable<Arg>, SqlError<EArg>> {
        let mut sql = map_intermediate_sql!(FromTable, self);
        sql.command.push_str(" FROM");
        sql.transition_static_from(tables)
    }
}
