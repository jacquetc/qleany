#pragma once

#include "qleany/qleany_export.h"
#include <QDebug>
#include <QHash>
#include <QMutex>
#include <QObject>
#include <qleany/contracts/repository/interface_repository.h>
#include <qleany/contracts/repository/interface_repository_provider.h>
#include <type_traits>

using namespace Qleany::Contracts::Repository;

namespace Qleany::Repository
{
class QLEANY_EXPORT RepositoryProvider : public QObject,
                                         public Qleany::Contracts::Repository::InterfaceRepositoryProvider
{
    Q_OBJECT

  public:
    static RepositoryProvider *instance();

    // InterfaceRepositoryProvider interface

  public:
    void registerRepository(const QString &name, InterfaceRepository *repository) override;

    InterfaceRepository *repository(const QString &name) override;

  private:
    RepositoryProvider() = default;
    RepositoryProvider(const RepositoryProvider &) = delete;
    RepositoryProvider &operator=(const RepositoryProvider &) = delete;

    QHash<QString, InterfaceRepository *> m_repositories;
    QMutex m_mutex;
    static QScopedPointer<RepositoryProvider> s_instance;
};
} // namespace Qleany::Repository
