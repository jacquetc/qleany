// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "repository/interface_repository.h"
#include "repository/interface_repository_provider.h"
#include "simple_example_persistence_export.h"
#include <QDebug>
#include <QHash>
#include <QMutex>
#include <QObject>
#include <type_traits>

using namespace Simple::Contracts::Repository;

namespace Simple::Persistence::Repository
{
class SIMPLE_EXAMPLE_PERSISTENCE_EXPORT RepositoryProvider
    : public QObject,
      public Qleany::Contracts::Repository::InterfaceRepositoryProvider
{
    Q_OBJECT

  public:
    static RepositoryProvider *instance();

    // InterfaceRepositoryProvider interface

  public:
    void registerRepository(const char *name, InterfaceRepository *repository) override;

    InterfaceRepository *repository(const char *name) override;

  private:
    RepositoryProvider() = default;
    RepositoryProvider(const RepositoryProvider &) = delete;
    RepositoryProvider &operator=(const RepositoryProvider &) = delete;

    QHash<QString, InterfaceRepository *> m_repositories;
    QMutex m_mutex;
    static QScopedPointer<RepositoryProvider> s_instance;
};
} // namespace Simple::Persistence::Repository