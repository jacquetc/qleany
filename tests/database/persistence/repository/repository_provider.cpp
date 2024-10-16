// This file was generated automatically by Qleany's generator, edit at your own risk! 
// If you do, be careful to not overwrite it when you run the generator again.
#include "repository_provider.h"

using namespace DatabaseTest::Persistence::Repository;

QScopedPointer<RepositoryProvider> RepositoryProvider::s_instance = QScopedPointer<RepositoryProvider>(nullptr);

RepositoryProvider *RepositoryProvider::instance()
{
    if (s_instance.isNull())
    {
        s_instance.reset(new RepositoryProvider());
    }

    return s_instance.data();
}

void RepositoryProvider::registerRepository(const char *name, InterfaceRepository *repository)
{
    QMutexLocker locker(&m_mutex);

    if (m_repositories.contains(QString::fromLatin1(name).toCaseFolded()))
    {
        qWarning() << "Repositories: m_repositories contains already this InterfaceRepository";
        return;
    }
    m_repositories.insert(QString::fromLatin1(name).toCaseFolded(), repository);
}

InterfaceRepository *RepositoryProvider::repository(const char *name)
{
    QMutexLocker locker(&m_mutex);
    auto repository = m_repositories.value(QString::fromLatin1(name).toCaseFolded(), nullptr);

    if (!repository)
    {
        qCritical() << "No repository registered for type" << QString::fromLatin1(name).toCaseFolded();
    }
    return repository;
}