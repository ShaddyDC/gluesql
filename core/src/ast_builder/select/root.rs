use {
    super::{join::JoinOperatorType, NodeData, Prebuild},
    crate::{
        ast::{SelectItem, TableAlias, TableFactor},
        ast_builder::{
            ExprList, ExprNode, FilterNode, GroupByNode, JoinNode, LimitNode, OffsetNode,
            OrderByExprList, OrderByNode, ProjectNode, SelectItemList,
        },
        result::Result,
    },
};

#[derive(Clone)]
pub struct SelectNode {
    table_name: String,
    table_alias: Option<String>,
}

impl SelectNode {
    pub fn new(table_name: String, table_alias: Option<String>) -> Self {
        Self {
            table_name,
            table_alias,
        }
    }

    pub fn filter<'a, T: Into<ExprNode<'a>>>(self, expr: T) -> FilterNode<'a> {
        FilterNode::new(self, expr)
    }

    pub fn group_by<'a, T: Into<ExprList<'a>>>(self, expr_list: T) -> GroupByNode<'a> {
        GroupByNode::new(self, expr_list)
    }

    pub fn offset<'a, T: Into<ExprNode<'a>>>(self, expr: T) -> OffsetNode<'a> {
        OffsetNode::new(self, expr)
    }

    pub fn limit<'a, T: Into<ExprNode<'a>>>(self, expr: T) -> LimitNode<'a> {
        LimitNode::new(self, expr)
    }

    pub fn project<'a, T: Into<SelectItemList<'a>>>(self, select_items: T) -> ProjectNode<'a> {
        ProjectNode::new(self, select_items)
    }

    pub fn order_by<'a, T: Into<OrderByExprList<'a>>>(self, order_by_exprs: T) -> OrderByNode<'a> {
        OrderByNode::new(self, order_by_exprs)
    }

    pub fn join<'a>(self, table_name: &str) -> JoinNode<'a> {
        JoinNode::new(self, table_name.to_owned(), None, JoinOperatorType::Inner)
    }

    pub fn join_as<'a>(self, table_name: &str, alias: &str) -> JoinNode<'a> {
        JoinNode::new(
            self,
            table_name.to_owned(),
            Some(alias.to_owned()),
            JoinOperatorType::Inner,
        )
    }

    pub fn left_join<'a>(self, table_name: &str) -> JoinNode<'a> {
        JoinNode::new(self, table_name.to_owned(), None, JoinOperatorType::Left)
    }

    pub fn left_join_as<'a>(self, table_name: &str, alias: &str) -> JoinNode<'a> {
        JoinNode::new(
            self,
            table_name.to_owned(),
            Some(alias.to_owned()),
            JoinOperatorType::Left,
        )
    }
}

impl Prebuild for SelectNode {
    fn prebuild(self) -> Result<NodeData> {
        let relation = TableFactor::Table {
            name: self.table_name,
            alias: self.table_alias.map(|name| TableAlias {
                name,
                columns: Vec::new(),
            }),
            index: None,
        };

        Ok(NodeData {
            projection: vec![SelectItem::Wildcard],
            relation,
            filter: None,
            group_by: vec![],
            having: None,
            order_by: vec![],
            offset: None,
            limit: None,
            joins: vec![],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ast_builder::{table, test, Build};

    #[test]
    fn select() {
        // select node -> build
        let actual = table("App").select().build();
        let expected = "SELECT * FROM App";
        test(actual, expected);

        let actual = table("Item").alias_as("i").select().build();
        let expected = "SELECT * FROM Item i";
        test(actual, expected);
    }
}
