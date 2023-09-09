#include "qleany/repository/repository_provider.h"

using namespace Qleany::Repository;

QScopedPointer<RepositoryProvider> RepositoryProvider::s_instance = QScopedPointer<RepositoryProvider>(nullptr);

RepositoryProvider *RepositoryProvider::instance()
{
    if (s_instance.isNull())
    {
        s_instance.reset(new RepositoryProvider());
    }

    return s_instance.data();
}

void RepositoryProvider::registerRepository(const QString &name, InterfaceRepository *repository)
{
    QMutexLocker locker(&m_mutex);

    if (m_repositories.contains(name.toCaseFolded()))
    {
        qWarning() << "Repositories: m_repositories contains already this InterfaceRepository";
        return;
    }
    m_repositories.insert(name.toCaseFolded(), repository);
}

InterfaceRepository *RepositoryProvider::repository(const QString &name)
{
    QMutexLocker locker(&m_mutex);
    auto repository = m_repositories.value(name.toCaseFolded(), nullptr);

    if (!repository)
    {
        qCritical() << "No repository registered for type" << name.toCaseFolded();
    }
    return repository;
}
