#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod flipper {

  /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    /// Определяет место хранения вашего контракта.
     /// Добавляем новые поля в приведенную ниже структуру по порядку
     /// чтобы добавить новые статические поля хранения в ваш контракт.
    #[ink(storage)]
    pub struct Flipper {
      /// Сохраняет одно значение `bool` в хранилище.
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Flipper {
      /// Конструктор, который инициализирует значение `bool` заданным `init_value`.
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Конструктор, который инициализирует значение `bool` значением `false`.
         ///
         /// Конструкторы могут делегировать полномочия другим конструкторам.
        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Сообщение, которое можно вызвать для экземпляров контрактов.
         /// Это переворачивает значение сохраненного `bool` от `true`
         /// в `false` и наоборот.
        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

/// Просто возвращает текущее значение нашего `bool`.
        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Модульные тесты в Rust обычно определяются внутри такого `#[cfg(test)]`
     /// модуль и тестовые функции отмечены атрибутом `#[test]`.
     /// Приведенный ниже код технически является обычным кодом Rust.
    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
      /// Импортирует все определения из внешней области, чтобы мы могли использовать их здесь.
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Мы проверяем, выполняет ли конструктор по умолчанию свою работу.
        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// Тестируем простой вариант использования нашего контракта.
        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
