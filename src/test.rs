#[cfg(test)]
mod tests {
    use crate::vault::{Cell, CellError, Item, Vault, VaultError};

    #[test]
    fn test_take_item_from_cell() {
        let mut cell = Cell {
            items: vec![
                Item {
                    name: "gold".to_string(),
                    size: 10,
                },
                Item {
                    name: "silver".to_string(),
                    size: 5,
                },
            ],
            capacity: 100,
            used_space: 15,
        };

        // берём предмет, который есть
        let item = cell.take("gold").expect("should take gold");
        assert_eq!(item.name, "gold");
        assert_eq!(item.size, 10);
        assert_eq!(cell.used_space, 5); // used_space уменьшился
        assert_eq!(cell.items.len(), 1);

        // берём предмет, который остался
        let item2 = cell.take("silver").expect("should take silver");
        assert_eq!(item2.name, "silver");
        assert_eq!(item2.size, 5);
        assert_eq!(cell.used_space, 0);
        assert!(cell.items.is_empty());

        // пытаемся взять несуществующий предмет
        let res = cell.take("diamond");
        assert!(matches!(res, Err(CellError::NotFound)));
    }

    #[test]
    fn test_take_item_from_vault() {
        let mut vault = Vault {
            cells: std::collections::HashMap::new(),
            capacity: 100,
        };

        vault.cells.insert(
            1,
            Cell {
                items: vec![
                    Item {
                        name: "gold".to_string(),
                        size: 10,
                    },
                    Item {
                        name: "silver".to_string(),
                        size: 5,
                    },
                ],
                capacity: 100,
                used_space: 15,
            },
        );

        // забираем существующий предмет
        let item = vault.take(1, "gold").expect("should take gold");
        assert_eq!(item.name, "gold");
        assert_eq!(item.size, 10);

        // забираем второй предмет
        let item2 = vault.take(1, "silver").expect("should take silver");
        assert_eq!(item2.name, "silver");
        assert_eq!(item2.size, 5);

        // пытаемся взять из пустой ячейки
        let res = vault.take(1, "diamond");
        assert!(matches!(res, Err(VaultError::ItemNotFound)));

        // пытаемся взять из несуществующей ячейки
        let res = vault.take(2, "gold");
        assert!(matches!(res, Err(VaultError::CellNotFound)));
    }
}
