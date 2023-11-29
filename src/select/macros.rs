pub struct Tables(pub(crate) &'static str);

#[allow(unused_macros)]
macro_rules! comma_separated {
    ($first:literal) => {
        $first
    };

    ($first:literal, $($column:literal),* $(,)?) => {
        concat!($first, $(", ", $column),*)
    };
}

#[allow(unused_imports)]
pub(super) use comma_separated;

/// Comma separated list of shared column names
#[allow(unused_macros)]
macro_rules! using {
    ($first:literal) => {
        concat!("USING (", $first, ")")
    };

    ($first:literal, $($column:literal),* $(,)?) => {
        concat!("USING (", $first, $(", ", $column),*, ")")
    };
}

#[allow(unused)]
pub(super) use using;

#[allow(unused_macros)]
macro_rules! logical_op {
    (AND) => {
        "AND"
    };
    (OR) => {
        "OR"
    };
}

#[allow(unused_imports)]
pub(super) use logical_op;

// TODO: support all the postgres dialect
// https://www.postgresql.org/docs/9.0/functions.html
#[allow(unused_macros)]
macro_rules! expression {
    ($simple:literal) => {
        $simple
    };

    (NOT $simple:literal) => {
        concat!("NOT ", $simple)
    };
}

#[allow(unused_imports)]
pub(super) use expression;

#[macro_export]
macro_rules! comparison {
    (=) => {
        "="
    };
    (!=) => {
        "<>"
    };
    (>) => {
        ">"
    };
    (<) => {
        "<"
    };
    (>=) => {
        ">="
    };
    (<=) => {
        "<="
    };
}

#[macro_export]
macro_rules! condition {
    ($a:literal $op:tt $b:literal) => {
        concat!($a, " ", $crate::select::comparison!($op), " ", $b)
    };

    ($a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)+) => {
        concat!(
            $crate::select::condition!($a $op $b),
            $(
                " ",
                $crate::select::logical_op!($logic_op),
                " ",
                $crate::select::condition!($ax $opx $bx)
            )+
        )
    };
}

#[macro_export]
macro_rules! static_tables {
    ($table:literal) => {
        $table
    };

    ($table:literal AS $alias:literal) => {
        concat!($table, " AS ", $alias)
    };

    ($ftable:literal $(AS $falias:literal)?, $($table:literal $(AS $alias:literal)?),* $(,)?) => {
        concat!($ftable $(, " AS ", $falias)?, $(", ", $table $(, " AS ", $alias)?),*)
    };
}

#[macro_export]
macro_rules! tables {
    ($column:literal) => {
        $crate::select::Tables($column)
    };

    ($column:literal AS $alias:literal) => {
        $crate::select::Tables($crate::select::static_tables!($column, AS, $alias))
    };

    ($fcolumn:literal $(AS $falias:literal)?, $($column:literal $(AS $alias:literal)?),* $(,)?) => {
        $crate::select::Tables($crate::select::static_tables!($fcolumn $(AS $falias)?, $($column $(AS $alias)?),*))
    };
}

#[macro_export]
macro_rules! join {
    (CROSS $table:literal) => {
        concat!("CROSS JOIN ", $table)
    };

    (INNER $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ON ",
            $crate::select::condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };

    (INNER $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "INNER JOIN ",
            $table,
            " ",
            $crate::select::using!($first, $($column),*),
        )
    };

    (LEFT $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ON ",
            $crate::select::condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };

    (LEFT $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "LEFT JOIN ",
            $table,
            " ",
            $crate::select::using!($first, $($column),*),
        )
    };

    (RIGHT $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ON ",
            $crate::select::condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };

    (RIGHT $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "RIGHT JOIN ",
            $table,
            " ",
            $crate::select::using!($first, $($column),*),
        )
    };

    (FULL $table:literal ON $a:literal $op:tt $b:literal $($logic_op:tt $ax:literal $opx:tt $bx:literal)*) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ON ",
            $crate::select::condition!($a $op $b $($logic_op $ax $opx $bx)*)
        )
    };

    (FULL $table:literal USING ($first:literal$(,)? $($column:literal),*)) => {
        concat!(
            "FULL JOIN ",
            $table,
            " ",
            $crate::select::using!($first, $($column),*)
        )
    };
}

#[macro_export]
macro_rules! group_by {
    ($first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY ", $first, $(", ", $column),*)
    };
    (ALL $first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY ALL ", $first, $(", ", $column),*)
    };
    (DISTINCT $first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!("GROUP BY DISTINCT ", $first, $(", ", $column),*)
    };
}

#[macro_export]
macro_rules! grouping_element {
    ($first:literal$(,)? $($column:literal),* $(,)?) => {
        concat!($first, $(", ", $column),*)
    };
    (ROLLUP ($first:literal$(,)? $($column:literal),*)) => {
        concat!("ROLLUP (", $first, $(", ", $column),*, ")")
    };
    (CUBE ($first:literal$(,)? $($column:literal),*)) => {
        concat!("CUBE (", $first, $(", ", $column),*, ")")
    };
    (GROUPING SETS ($first:literal$(,)? $($column:literal),*)) => {
        concat!("GROUPING SETS (", $first, $(", ", $column),*, ")")
    };
}

// TODO: support ORDER BY x DESC, y DESC NULL FIRST, z ASC, a USING > NULL LAST
#[allow(unused_macros)]
macro_rules! order_by {
    ($col_expr:literal ASC) => {
        concat!("ORDER BY ", $col_expr, " ASC")
    };
    ($col_expr:literal ASC NULLS FIRST) => {
        concat!("ORDER BY ", $col_expr, " ASC NULLS FIRST")
    };
    ($col_expr:literal ASC NULLS LAST) => {
        concat!("ORDER BY ", $col_expr, " ASC NULLS LAST")
    };

    ($col_expr:literal DESC) => {
        concat!("ORDER BY ", $col_expr, " DESC")
    };
    ($col_expr:literal DESC NULLS FIRST) => {
        concat!("ORDER BY ", $col_expr, " DESC NULLS FIRST")
    };
    ($col_expr:literal DESC NULLS LAST) => {
        concat!("ORDER BY ", $col_expr, " DESC NULLS LAST")
    };

    ($col_expr:literal USING $op:tt) => {
        concat!(
            "ORDER BY ",
            $col_expr,
            " USING ",
            $crate::select::comparison!($op)
        )
    };
}

// TODO: https://www.postgresql.org/docs/current/sql-select.html#SQL-LIMIT
#[allow(unused_macros)]
macro_rules! limit {
    (ALL) => {
        "LIMIT ALL"
    };
}

// TODO: create select!() macro to build a sql command in compile-time

pub use static_tables;

pub use comparison;
pub use condition;
pub use join;
pub use tables;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comma_separated_test() {
        assert_eq!(comma_separated!("id"), "id");
        assert_eq!(comma_separated!("'quoted column'",), "'quoted column'");
        assert_eq!(comma_separated!("id", "name"), "id, name");
        assert_eq!(comma_separated!("id", "name", "age",), "id, name, age");
        assert_eq!(
            comma_separated!("id", "name", "age", "email"),
            "id, name, age, email"
        );
    }

    #[test]
    fn static_tables_test() {
        assert_eq!(static_tables!("user"), "user");
        assert_eq!(static_tables!("user" AS "u"), "user AS u");
        assert_eq!(static_tables!("'quoted table'",), "'quoted table'");
        assert_eq!(
            static_tables!("'quoted table'" AS "'other'",),
            "'quoted table' AS 'other'"
        );
        assert_eq!(static_tables!("user", "customer"), "user, customer");
        assert_eq!(
            static_tables!("user", "customer", "organization",),
            "user, customer, organization"
        );
        assert_eq!(
            static_tables!("user", "customer" AS "c", "organization", "product" AS "p",),
            "user, customer AS c, organization, product AS p"
        );
    }

    #[test]
    fn using_test() {
        assert_eq!(using!("id"), "USING (id)");
        assert_eq!(using!("id", "customer_id"), "USING (id, customer_id)");
        assert_eq!(
            using!("id", "sale_id", "customer_id",),
            "USING (id, sale_id, customer_id)"
        );
    }

    #[test]
    fn join_test() {
        assert_eq!(join!(CROSS "user"), "CROSS JOIN user");

        assert_eq!(
            join!(INNER "user" USING ("id", "department")),
            "INNER JOIN user USING (id, department)"
        );
        assert_eq!(
            join!(INNER "user" ON "user.id" = "access_history.user_id"),
            "INNER JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            join!(INNER "user" ON "user.id" = "access_history.user_id" AND "user.updated" < "access_history.created"),
            "INNER JOIN user ON user.id = access_history.user_id AND user.updated < access_history.created"
        );

        assert_eq!(
            join!(LEFT "user" USING ("id", "department")),
            "LEFT JOIN user USING (id, department)"
        );
        assert_eq!(
            join!(LEFT "user" ON "user.id" = "access_history.user_id"),
            "LEFT JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            join!(LEFT "user" ON "user.id" = "access_history.user_id" OR "user.updated" < "access_history.created"),
            "LEFT JOIN user ON user.id = access_history.user_id OR user.updated < access_history.created"
        );

        assert_eq!(
            join!(RIGHT "user" USING ("id", "department")),
            "RIGHT JOIN user USING (id, department)"
        );
        assert_eq!(
            join!(RIGHT "user" ON "user.id" = "access_history.user_id"),
            "RIGHT JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            join!(RIGHT "user" ON "user.id" = "access_history.user_id" AND "user.updated" < "access_history.created"),
            "RIGHT JOIN user ON user.id = access_history.user_id AND user.updated < access_history.created"
        );

        assert_eq!(
            join!(FULL "user" USING ("id", "department")),
            "FULL JOIN user USING (id, department)"
        );
        assert_eq!(
            join!(FULL "user" ON "user.id" = "access_history.user_id"),
            "FULL JOIN user ON user.id = access_history.user_id"
        );
        assert_eq!(
            join!(FULL "user" ON "user.id" = "access_history.user_id" OR "user.updated" < "access_history.created"),
            "FULL JOIN user ON user.id = access_history.user_id OR user.updated < access_history.created"
        );
    }
}

// NOTE: stringfy! macro
